use std::path::PathBuf;

use clap::Parser;
use gltf::mesh::util::ReadIndices;
use gltf::{Gltf, Semantic};

#[derive(Parser)]
#[clap(author, version, about)]
struct Args {
    #[clap(short, long, value_parser, help = "Path of a GLTF file.")]
    path: PathBuf,
}

fn main() {
    let args = Args::parse();

    // TODO: better error handling
    let (document, buffers, images) = gltf::import(args.path.as_path()).unwrap();
    let get_buffer_data = |buffer: gltf::Buffer| buffers.get(buffer.index()).map(|x| &*x.0);

    for mesh in document.meshes() {
        for primitive in mesh.primitives() {
            let positions: Vec<[f32; 3]> = primitive
                .reader(get_buffer_data)
                .read_positions()
                .unwrap()
                .collect();
            let indices: Vec<u16> = match primitive.reader(get_buffer_data).read_indices().unwrap()
            {
                ReadIndices::U16(indices) => indices.collect(),
                _ => panic!("Shit"), // TODO
            };

            println!("X {:?}", indices,);
            println!("Y {:?}", positions);
        }
    }
}
