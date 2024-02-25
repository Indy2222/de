use bevy::prelude::*;

use self::packed::{PackedId, PackedVec};

mod packed;

struct Tree {
    half_size: Vec2,
    root: Child,
    nodes: PackedVec<Node>,
    buckets: PackedVec<Bucket>,
}

impl Tree {
    fn insert(&mut self, entity: Entity, point: Vec2) {
        let mut target = self.root;

        while let Child::Node(id) = target {
            // TODO find child
        }

        let Child::Bucket(mut bucket_id) = target else {
            panic!("Target is not a bucket.");
        };

        let Some(bucket) = self.buckets.get_mut(bucket_id) else {
            panic!("Target bucket ID points to a non-existent bucket: {bucket_id}");
        };

        if bucket.is_full() {
            let removed = self.buckets.remove(bucket_id);

            let grandparent = removed.item.parent;
            // TODO better error
            let parent = self.nodes.next_id().unwrap();

            // TODO grandparent -> new child

            Node::from_buckets(
                grandparent,
                self.buckets.push(Bucket::new(parent)),
                self.buckets.push(Bucket::new(parent)),
                self.buckets.push(Bucket::new(parent)),
                self.buckets.push(Bucket::new(parent)),
            );

            // TODO find correct bucket ID
            // bucket_id = bucket_id;
        }

        let Some(bucket) = self.buckets.get_mut(bucket_id) else {
            panic!("Target bucket ID points to a non-existent bucket: {bucket_id}");
        };

        bucket.insert(entity, point);
    }
}

struct Node {
    // TODO custom unit type for NonZeroU16
    parent: Option<PackedId>,
    children: [Child; 4],
}

impl Node {
    fn from_buckets(
        parent: PackedId,
        top_left: PackedId,
        top_right: PackedId,
        bottom_left: PackedId,
        bottom_right: PackedId,
    ) -> Self {
        Self {
            parent: Some(parent),
            children: [
                Child::Bucket(top_left),
                Child::Bucket(top_right),
                Child::Bucket(bottom_left),
                Child::Bucket(bottom_right),
            ],
        }
    }

    fn x(&self, min: Vec2, max: Vec2, point: Vec2) -> (Child, Vec2, Vec2) {
        let mid = min.lerp(max, 0.5);
        match mid.cmplt(point).bitmask() {
            // top-left
            0 => (self.children[0], min, mid),
            // top-right
            1 => (
                self.children[1],
                Vec2::new(mid.x, min.y),
                Vec2::new(max.x, mid.y),
            ),
            // TODO
            // bottom-left
            2 => {}
            // bottom-right
            _ => {}
        }
    }
}

#[derive(Clone, Copy)]
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
