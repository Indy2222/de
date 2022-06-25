use std::path::PathBuf;

use clap::Parser;
use glam::{Mat4, Vec3};
use parry3d::{
    bounding_volume::AABB,
    math::{Isometry, Point},
    transformation,
};

#[derive(Parser)]
#[clap(author, version, about)]
struct Args {
    #[clap(short, long, value_parser, help = "Path of a GLTF file.")]
    path: PathBuf,
}

fn main() {
    let args = Args::parse();

    let (document, buffers, _images) = match gltf::import(args.path.as_path()) {
        Ok(loaded) => loaded,
        Err(err) => panic!("GLTF loading error: {:?}", err),
    };
    let get_buffer_data = |buffer: gltf::Buffer| buffers.get(buffer.index()).map(|x| &*x.0);

    let (min, max) = document
        .scenes()
        .flat_map(|scene| scene.nodes())
        .filter_map(|node| match node.mesh() {
            Some(mesh) => {
                let trans = Mat4::from_cols_array_2d(&node.transform().matrix());
                //let trans = trans.inverse();
                Some((mesh, trans))
            }
            None => None,
        })
        .flat_map(|(mesh, trans)| mesh.primitives().map(move |p| (p, trans.clone())))
        .flat_map(|(primitive, trans)| {
            primitive
                .reader(get_buffer_data)
                .read_positions()
                .unwrap()
                .map(move |i| (i, trans.clone()))
        })
        .fold(
            (
                [f32::INFINITY, f32::INFINITY, f32::INFINITY],
                [f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY],
            ),
            |mut acc, (item, trans)| {
                let vec = trans * Vec3::from(item).extend(1.);

                for (i, &coord) in [vec.x, vec.y, vec.z].iter().enumerate() {
                    acc.0[i] = acc.0[i].min(coord);
                    acc.1[i] = acc.1[i].max(coord);
                }
                acc
            },
        );

    let (positions, indices) = AABB::new(Point::from(min), Point::from(max)).to_trimesh();
    println!("Positions: {:?}", positions);
    println!("Indices: {:?}", indices);
}
