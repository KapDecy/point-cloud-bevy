use bevy::math::Vec4Swizzles;
use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::mesh::VertexAttributeValues;
use bevy::render::render_resource::PrimitiveTopology;
use ply_rs::parser;
use ply_rs::ply;

pub(crate) struct PlyPlugin;

// element vertex 1877661
// property float x
// property float y
// property float z
// property float nx
// property float ny
// property float nz
// property uchar red
// property uchar green
// property uchar blue

/// We know, what data we want to read, hence we can be more efficient by loading the data into structs.
#[derive(Debug, Clone)] // not necessary for parsing, only for println at end of example.
struct Vertex {
    x: f32,
    y: f32,
    z: f32,
    nx: f32,
    ny: f32,
    nz: f32,
    red: u8,
    green: u8,
    blue: u8,
}

// The structs need to implement the PropertyAccess trait, otherwise the parser doesn't know how to write to them.
// Most functions have default, hence you only need to implement, what you expect to need.

impl ply::PropertyAccess for Vertex {
    fn new() -> Self {
        Vertex {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            nx: 0.0,
            ny: 0.0,
            nz: 0.0,
            red: 0,
            green: 0,
            blue: 0,
        }
    }
    fn set_property(&mut self, key: String, property: ply::Property) {
        match (key.as_ref(), property) {
            ("x", ply::Property::Float(v)) => self.x = v,
            ("y", ply::Property::Float(v)) => self.y = v,
            ("z", ply::Property::Float(v)) => self.z = v,
            ("nx", ply::Property::Float(v)) => self.nx = v,
            ("ny", ply::Property::Float(v)) => self.ny = v,
            ("nz", ply::Property::Float(v)) => self.nz = v,
            ("red", ply::Property::UChar(v)) => self.red = v,
            ("green", ply::Property::UChar(v)) => self.green = v,
            ("blue", ply::Property::UChar(v)) => self.blue = v,
            (k, _) => panic!("Vertex: Unexpected key/value combination: key: {}", k),
        }
    }
}

impl Plugin for PlyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::setup).add_system(rotate);
    }
}

fn rotate(mut query: Query<&mut Transform, With<CloudComponent>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_z(time.delta_seconds() / 6.);
    }
}

impl PlyPlugin {
    /// Demonstrates simplest use case for reading from a file.
    fn setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        // set up a reader, in this a file.
        let path = "point_cloud.ply";
        let f = std::fs::File::open(path).unwrap();
        // The header of a ply file consists of ascii lines, BufRead provides useful methods for that.
        let mut f = std::io::BufReader::new(f);

        // Create a parser for each struct. Parsers are cheap objects.
        let vertex_parser = parser::Parser::<Vertex>::new();

        // lets first consume the header
        // We also could use `face_parser`, The configuration is a parser's only state.
        // The reading position only depends on `f`.
        let header = vertex_parser.read_header(&mut f).unwrap();

        // Depending on the header, read the data into our structs..
        let mut vertex_list = Vec::new();
        for (_ignore_key, element) in &header.elements {
            // we could also just parse them in sequence, but the file format might change
            match element.name.as_ref() {
                "vertex" => {
                    vertex_list = vertex_parser
                        .read_payload_for_element(&mut f, element, &header)
                        .unwrap();
                }
                _ => panic!("Enexpeced element!"),
            }
        }

        // proof that data has been read
        println!("header: {:#?}", header);
        println!("vertex list: {:#?}", vertex_list[0]);

        let mut my_meshes = vec![];
        let mut transforms = vec![];
        // let color = Color::rgb_u8(0,0,0).as_rgba_f32()

        for vertex in vertex_list[0..400_000].iter() {
            let mut mesh: Mesh = shape::Icosphere {
                radius: 0.06,
                subdivisions: 0,
            }
            .into();
            // let mut mesh: Mesh = shape::Cube { size: 0.1 }.into();

            let color = Color::rgb_u8(vertex.red, vertex.green, vertex.blue);

            let mut colors: Vec<[f32; 4]> = vec![];
            for _ in 0..mesh.count_vertices() {
                colors.push(color.as_rgba_f32());
            }
            mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
            my_meshes.push(mesh);

            transforms.push(Transform::from_xyz(vertex.x, vertex.y, vertex.z));

            // commands.spawn(PbrBundle {
            //     mesh: meshes.add(
            //         Mesh::try_from(shape::Icosphere {
            //             radius: 0.1,
            //             subdivisions: 1,
            //         })
            //         .unwrap(),
            //     ),
            //     material: materials.add(StandardMaterial {
            //         base_color: Color::rgb_u8(vertex.red, vertex.green, vertex.blue),
            //         // vary key PBR parameters on a grid of spheres to show the effect
            //         metallic: 1.,
            //         perceptual_roughness: 1.,
            //         ..default()
            //     }),
            //     transform: Transform::from_xyz(vertex.x, vertex.y, vertex.z),
            //     ..default()
            // });
        }
        let main_mesh = combine_meshes(my_meshes, transforms, true, false, true, true);

        commands.spawn((
            PbrBundle {
                mesh: meshes.add(main_mesh),
                material: materials.add(StandardMaterial::default()),
                ..default()
            },
            CloudComponent,
        ));
    }
}

#[derive(Component)]
struct CloudComponent;

fn combine_meshes(
    meshes: Vec<Mesh>,
    transforms: Vec<Transform>,
    use_normals: bool,
    use_tangents: bool,
    use_uvs: bool,
    use_colors: bool,
) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut tangets: Vec<[f32; 4]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();
    let mut colors: Vec<[f32; 4]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    let mut indices_offset = 0;

    if meshes.len() != transforms.len() {
        panic!(
            "meshes.len({}) != transforms.len({})",
            meshes.len(),
            transforms.len()
        );
    }

    for (mesh, trans) in meshes.iter().zip(transforms) {
        if let Indices::U32(mesh_indices) = mesh.indices().unwrap() {
            let mat = trans.compute_matrix();

            let positions_len;

            if let Some(VertexAttributeValues::Float32x3(vert_positions)) =
                &mesh.attribute(Mesh::ATTRIBUTE_POSITION)
            {
                positions_len = vert_positions.len();
                for p in vert_positions {
                    positions.push(mat.transform_point3(Vec3::from(*p)).into());
                }
            } else {
                panic!("no positions")
            }

            if use_uvs {
                if let Some(VertexAttributeValues::Float32x2(vert_uv)) =
                    &mesh.attribute(Mesh::ATTRIBUTE_UV_0)
                {
                    for uv in vert_uv {
                        uvs.push(*uv);
                    }
                } else {
                    panic!("no uvs")
                }
            }

            if use_normals {
                // Comment below taken from mesh_normal_local_to_world() in mesh_functions.wgsl regarding
                // transform normals from local to world coordinates:

                // NOTE: The mikktspace method of normal mapping requires that the world normal is
                // re-normalized in the vertex shader to match the way mikktspace bakes vertex tangents
                // and normal maps so that the exact inverse process is applied when shading. Blender, Unity,
                // Unreal Engine, Godot, and more all use the mikktspace method. Do not change this code
                // unless you really know what you are doing.
                // http://www.mikktspace.com/

                let inverse_transpose_model = mat.inverse().transpose();
                let inverse_transpose_model = Mat3 {
                    x_axis: inverse_transpose_model.x_axis.xyz(),
                    y_axis: inverse_transpose_model.y_axis.xyz(),
                    z_axis: inverse_transpose_model.z_axis.xyz(),
                };

                if let Some(VertexAttributeValues::Float32x3(vert_normals)) =
                    &mesh.attribute(Mesh::ATTRIBUTE_NORMAL)
                {
                    for n in vert_normals {
                        normals.push(
                            inverse_transpose_model
                                .mul_vec3(Vec3::from(*n))
                                .normalize_or_zero()
                                .into(),
                        );
                    }
                } else {
                    panic!("no normals")
                }
            }

            if use_tangents {
                if let Some(VertexAttributeValues::Float32x4(vert_tangets)) =
                    &mesh.attribute(Mesh::ATTRIBUTE_TANGENT)
                {
                    for t in vert_tangets {
                        tangets.push(*t);
                    }
                } else {
                    panic!("no tangets")
                }
            }

            if use_colors {
                if let Some(VertexAttributeValues::Float32x4(vert_colors)) =
                    &mesh.attribute(Mesh::ATTRIBUTE_COLOR)
                {
                    for c in vert_colors {
                        colors.push(*c);
                    }
                } else {
                    panic!("no colors")
                }
            }

            for i in mesh_indices {
                indices.push(*i + indices_offset);
            }
            indices_offset += positions_len as u32;
        }
    }

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);

    if use_normals {
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    }

    if use_tangents {
        mesh.insert_attribute(Mesh::ATTRIBUTE_TANGENT, tangets);
    }

    if use_uvs {
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    }

    if use_colors {
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    }

    mesh.set_indices(Some(Indices::U32(indices)));

    mesh
}
