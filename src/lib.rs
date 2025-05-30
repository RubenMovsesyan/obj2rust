use std::{env, path::Path};

use mesh::Vertex;
use proc_macro::TokenStream;
use quote::quote;
use read_file::read_lines;
use syn::{LitStr, parse_macro_input};

mod mesh;
mod read_file;

#[proc_macro]
pub fn obj_2_rust(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let file_name = input.value();

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let manifest_path = Path::new(&manifest_dir);
    let file_path = manifest_path.with_file_name(file_name);
    let lines = read_lines(file_path).unwrap();

    let mut vertices: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut uv_coords: Vec<[f32; 2]> = Vec::new();

    let mut model_vertices: Vec<(f32, f32, f32)> = Vec::new();

    let mut indices: Vec<u32> = Vec::new();

    for line in lines.map_while(Result::ok) {
        let line_split = line.split_whitespace().collect::<Vec<_>>();

        if line_split.is_empty() {
            continue;
        }

        match line_split[0] {
            "o" => {
                // TODO: implement
            }
            // Vertex position
            "v" => {
                vertices.push([
                    line_split[1].parse::<f32>().unwrap(),
                    line_split[2].parse::<f32>().unwrap(),
                    line_split[3].parse::<f32>().unwrap(),
                ]);
            }
            // UV Coordinate
            "vt" => {
                uv_coords.push([
                    1.0 - line_split[1].parse::<f32>().unwrap(),
                    1.0 - line_split[2].parse::<f32>().unwrap(),
                ]);
            }
            // Normals
            "vn" => {
                normals.push([
                    line_split[1].parse::<f32>().unwrap(),
                    line_split[2].parse::<f32>().unwrap(),
                    line_split[3].parse::<f32>().unwrap(),
                ]);
            }
            // face
            "f" => {
                // Assuming triangulated
                for vertex_info in line_split[1..3].iter() {
                    let vertex_info_split = vertex_info.split('/').collect::<Vec<_>>();

                    // Get the indices of each vertex, uv, and normal for the face
                    let (vertex_index, uv_index, normal_index) = (
                        vertex_info_split[0].parse::<usize>().unwrap() - 1,
                        vertex_info_split[1].parse::<usize>().unwrap() - 1,
                        vertex_info_split[2].parse::<usize>().unwrap() - 1,
                    );

                    // model_vertices.push(Vertex {
                    //     position: vertices[vertex_index],
                    //     normal: normals[normal_index],
                    //     uv: uv_coords[uv_index],
                    // });
                    model_vertices.push(vertices[vertex_index].into());

                    indices.push(vertex_index as u32);
                }
            }
            _ => {}
        }
    }

    let vertex_tokens = model_vertices
        .into_iter()
        .map(|vertex| {
            // let position = &vertex.position;
            // let normal = &vertex.normal;
            // let uv = &vertex.uv;
            let (x, y, z) = vertex;

            quote! {
                (#x, #y, #z)
            }
        })
        .collect::<Vec<_>>();

    let index_tokens = indices
        .iter()
        .map(|index| {
            let model_index = &index;

            quote! {
                #model_index
            }
        })
        .collect::<Vec<_>>();

    let size = index_tokens.len();

    let combined = quote! {
        ([#(#vertex_tokens),*], [#(#index_tokens),*], #size)
    };

    combined.into()
}
