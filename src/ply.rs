use bevy::prelude::*;
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
        app.add_startup_system(Self::setup);
    }
}

impl PlyPlugin {
    /// Demonstrates simplest use case for reading from a file.
    fn setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        asset_server: Res<AssetServer>,
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

        for vertex in vertex_list[0..=100_000].iter() {
            commands.spawn(PbrBundle {
                mesh: meshes.add(
                    Mesh::try_from(shape::Icosphere {
                        radius: 0.1,
                        subdivisions: 1,
                    })
                    .unwrap(),
                ),
                material: materials.add(StandardMaterial {
                    base_color: Color::rgb_u8(vertex.red, vertex.green, vertex.blue),
                    // vary key PBR parameters on a grid of spheres to show the effect
                    metallic: 1.,
                    perceptual_roughness: 1.,
                    ..default()
                }),
                transform: Transform::from_xyz(vertex.x, vertex.y, vertex.z),
                ..default()
            });
        }

        // commands
        //     .spawn(PointCloudBevyComponent)
        //     .insert(Name::new("PointCloudBevyPlugin Root"))
        //     .insert(SpatialBundle::default())
        //     .with_children(|commands| {
        //         commands
        //             .spawn(SceneBundle {
        //                 scene: asset_server.load("cube.glb#Scene0"),
        //                 transform: Transform::from_xyz(0.0, 0.0, 0.0).with_rotation(
        //                     Quat::from_euler(
        //                         EulerRot::XYZ,
        //                         22.5 * TAU / 360.0,
        //                         45.0 * TAU / 360.0,
        //                         0.0,
        //                     ),
        //                 ),
        //                 ..default()
        //             })
        //             .insert(Name::new("PointCloudBevyPlugin Scene"));
        //     });
    }
}

#[derive(Debug, Component)]
pub struct PlyComponent {
    vertex: Vertex,
}
