use bevy::prelude::*;

use self::{
    packed::{PackedId, PackedVec},
    path::Half,
};
use crate::point::path::BinaryPath;

mod packed;
mod path;

struct Tree {
    half_size: Vec2,
    nodes: PackedVec<Node>,
    buckets: PackedVec<Bucket>,
}

impl Tree {
    fn find_bucket(&self, point: Vec2) -> (BinaryPath, PackedId) {
        let mut path = BinaryPath::new(self.half_size, point);

        let mut target = Child::Node(PackedId::FIRST);
        loop {
            match target {
                Child::Bucket(bucket_id) => break (path, bucket_id),
                Child::Node(node_id) => {
                    target = self.nodes.get(node_id).unwrap().get_child(path.step());
                }
            }
        }
    }

    fn remove_bucket(&mut self, bucket_id: PackedId) -> Bucket {
        let removed = self.buckets.remove(bucket_id);

        if let Some(swap) = removed.swap {
            let old = Child::Bucket(swap.old);
            let new = Child::Bucket(swap.new);

            let parent_id = self.buckets.get(swap.new).unwrap().parent;
            self.nodes
                .get_mut(parent_id)
                .unwrap()
                .replace_child(old, new);
        }

        removed.item
    }

    fn insert(&mut self, entity: Entity, point: Vec2) {
        let (path, bucket_id) = self.find_bucket(point);
        let bucket = self.buckets.get_mut(bucket_id).unwrap();

        if bucket.is_full() {
            let bucket = self.remove_bucket(bucket_id);

            let grandparent_id = bucket.parent;
            let parent_id = self.nodes.next_id().unwrap();

            self.nodes
                .get_mut(grandparent_id)
                .unwrap()
                .replace_child(Child::Bucket(bucket_id), Child::Node(parent_id));

            let parent = Node::from_buckets(
                grandparent_id,
                self.buckets.push(Bucket::new(parent_id)),
                self.buckets.push(Bucket::new(parent_id)),
            );

            let new_parent_id = self.nodes.push(parent);
            debug_assert_eq!(new_parent_id, parent_id);

            // TODO split entities from bucket to new children
            // TODO include the new entity into the entities to be split
            // TODO this must be done in a loop if entities happen to fall to the same bucket
        } else {
            bucket.insert(entity, point);
        }
    }
}

struct Node {
    parent: Option<PackedId>,
    children: [Child; 2],
}

impl Node {
    fn from_buckets(parent: PackedId, top_left: PackedId, bottom_right: PackedId) -> Self {
        Self {
            parent: Some(parent),
            children: [Child::Bucket(top_left), Child::Bucket(bottom_right)],
        }
    }

    fn replace_child(&self, from: Child, to: Child) {
        for i in 0..self.children.len() {
            if self.children[i] == from {
                self.children[i] = to;
                break;
            }
        }
    }

    fn get_child(&self, half: Half) -> Child {
        match half {
            Half::TopLeft => self.children[0],
            Half::BottomRight => self.children[1],
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Child {
    Node(PackedId),
    Bucket(PackedId),
}

struct Bucket {
    parent: PackedId,
    len: u8,
    items: [Item; Self::MAX_LEN as usize],
}

impl Bucket {
    const MAX_LEN: u8 = 16;

    fn new(parent: PackedId) -> Self {
        Self {
            parent,
            len: 0,
            items: [Item::PLACEHOLDER; Self::MAX_LEN as usize],
        }
    }

    fn insert(&mut self, entity: Entity, point: Vec2) {
        debug_assert!(!self.is_full());
        self.items[self.len as usize] = Item { entity, point };
        self.len += 1;
    }

    fn is_full(&self) -> bool {
        Self::MAX_LEN == self.len
    }
}

struct Item {
    entity: Entity,
    point: Vec2,
}

impl Item {
    const PLACEHOLDER: Self = Self {
        entity: Entity::PLACEHOLDER,
        point: Vec2::ZERO,
    };
}
