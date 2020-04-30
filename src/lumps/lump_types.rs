use crate::lumps::LumpReader;
use regex::Regex;

use std::collections::HashMap;

pub trait BspParseable {
    fn from_reader(data: &mut LumpReader) -> Self;
}

pub type Entity = HashMap<String, String>;

type Vector = (f32, f32, f32);

impl BspParseable for Vector {
    fn from_reader(data: &mut LumpReader) -> Self {
        (data.read_f32(), data.read_f32(), data.read_f32())
    }
}

#[derive(Debug)]
pub struct Plane {
    /// Normal vector
    pub normal: Vector,
    /// Distance from the origin (0,0,0)
    pub dist_from_origin: f32,
    /// Plane axis identifier
    pub r#type: i32,
}

impl BspParseable for Plane {
    fn from_reader(data: &mut LumpReader) -> Self {
        Self {
            normal: Vector::from_reader(data),

            dist_from_origin: data.read_f32(),
            r#type: data.read_i32(),
        }
    }
}

#[derive(Debug)]
pub struct TexData {
    /// RGB Reflectivity
    pub reflectivity: Vector,
    /// Index into TexdataStringTable
    pub texdata_string_table_index: i32,
    /// Source image width
    pub width: i32,
    /// Source image height
    pub height: i32,
    /// Displayed image width
    pub view_width: i32,
    /// Displayed image height
    pub view_height: i32,
}

impl BspParseable for TexData {
    fn from_reader(data: &mut LumpReader) -> Self {
        Self {
            reflectivity: (data.read_f32(), data.read_f32(), data.read_f32()),

            texdata_string_table_index: data.read_i32(),

            width: data.read_i32(),
            height: data.read_i32(),
            view_width: data.read_i32(),
            view_height: data.read_i32(),
        }
    }
}

#[derive(Debug)]
pub struct Vertex {
    /// X coordinate
    pub x: f32,
    /// Y coordinate
    pub y: f32,
    /// Z coordinate
    pub z: f32,
}

impl BspParseable for Vertex {
    fn from_reader(data: &mut LumpReader) -> Self {
        Self {
            x: data.read_f32(),
            y: data.read_f32(),
            z: data.read_f32(),
        }
    }
}

#[derive(Debug)]
pub struct Node {
    /// Index into the plane array
    pub plane_num: i32,
    /// Negative numbers mean -(leaf + 1), not nodes
    pub children: [i32; 2],
    /// For frustrum culling
    pub mins: [i16; 3],
    /// For frustrum culling
    pub maxs: [i16; 3],
    /// Index into the face array
    pub first_face: u16,
    /// Number of faces (counting both sides)
    pub num_faces: u16,
    /// If all leaves below this node are in the same area, this is the area distance, otherwise it is -1
    pub area: i16,
    /// Pad to be 32 bytes
    pub padding: i16,
}

impl BspParseable for Node {
    fn from_reader(data: &mut LumpReader) -> Self {
        Self {
            plane_num: data.read_i32(),
            children: [data.read_i32(), data.read_i32()],
            mins: [data.read_i16(), data.read_i16(), data.read_i16()],
            maxs: [data.read_i16(), data.read_i16(), data.read_i16()],
            first_face: data.read_u16(),
            num_faces: data.read_u16(),
            area: data.read_i16(),
            padding: data.read_i16(),
        }
    }
}

#[derive(Debug)]
pub struct TexInfo {
    /// [s/t]/[xyz offset]
    pub texture_vecs: [[f32; 4]; 2],
    /// [s/t]/[xyz offset]
    pub lightmap_vecs: [[f32; 4]; 2],
    /// Miptex flag overrides
    pub flags: i32,
    /// Pointer to texture name/size/etc
    pub tex_data: i32,
}

impl BspParseable for TexInfo {
    fn from_reader(data: &mut LumpReader) -> Self {
        Self {
            texture_vecs: [
                [
                    data.read_f32(),
                    data.read_f32(),
                    data.read_f32(),
                    data.read_f32(),
                ],
                [
                    data.read_f32(),
                    data.read_f32(),
                    data.read_f32(),
                    data.read_f32(),
                ],
            ],
            lightmap_vecs: [
                [
                    data.read_f32(),
                    data.read_f32(),
                    data.read_f32(),
                    data.read_f32(),
                ],
                [
                    data.read_f32(),
                    data.read_f32(),
                    data.read_f32(),
                    data.read_f32(),
                ],
            ],
            flags: data.read_i32(),
            tex_data: data.read_i32(),
        }
    }
}

#[derive(Debug)]
pub struct Face {
    /// The plane number
    pub plane_num: u16,
    /// Faces opposite to the node's plane direction
    pub side: u8,
    /// 1 if on a node, 0 if in leaf
    pub on_node: u8,
    /// Index into surfedges
    pub first_edge: i32,
    /// Number of surfedges
    pub num_edges: i16,
    /// Texture info
    pub texinfo: i16,
    /// Displacement info
    pub displacement_info: i16,
    /// ???
    pub surface_fog_volume_id: i16,
    /// Switchable lighting info
    pub styles: [u8; 4],
    /// Offset into lightmap lump
    pub light_offset: i32,
    /// Face area in units^2
    pub area: f32,
    /// Texture lighting info
    pub lightmap_texture_mins_in_luxels: [i32; 2],
    /// Texture lighting info
    pub lightmap_texture_size_in_luxels: [i32; 2],
    /// Original face this was split from
    pub original_face: i32,
    /// Primitives
    pub num_primitives: u16,
    /// ???
    pub first_primitave_id: u16,
    /// Lightmap smoothing group
    pub smoothing_groups: u32,
}

impl BspParseable for Face {
    fn from_reader(data: &mut LumpReader) -> Self {
        Self {
            plane_num: data.read_u16(),
            side: data.read_u8(),
            on_node: data.read_u8(),
            first_edge: data.read_i32(),
            num_edges: data.read_i16(),
            texinfo: data.read_i16(),
            displacement_info: data.read_i16(),
            surface_fog_volume_id: data.read_i16(),
            styles: [
                data.read_u8(),
                data.read_u8(),
                data.read_u8(),
                data.read_u8(),
            ],
            light_offset: data.read_i32(),
            area: data.read_f32(),
            lightmap_texture_mins_in_luxels: [data.read_i32(), data.read_i32()],
            lightmap_texture_size_in_luxels: [data.read_i32(), data.read_i32()],
            original_face: data.read_i32(),
            num_primitives: data.read_u16(),
            first_primitave_id: data.read_u16(),
            smoothing_groups: data.read_u32(),
        }
    }
}

/// Calculating normal RBG:
/// R: R * 2^(exponent)
/// G: G * 2^(exponent)
/// B: B * 2^(exponent)
#[derive(Debug)]
pub struct LightmapSample {
    /// Red value
    pub r: u8,
    /// Green value
    pub g: u8,
    /// Blue value
    pub b: u8,
    /// Exponent for 2^
    pub exponent: i8,
}

impl BspParseable for LightmapSample {
    fn from_reader(data: &mut LumpReader) -> Self {
        Self {
            r: data.read_u8(),
            g: data.read_u8(),
            b: data.read_u8(),
            exponent: data.read_i8(),
        }
    }
}

#[derive(Debug)]
pub struct OccluderData {
    pub flags: i32,
    /// Index into OccluderPolyData
    pub first_poly: i32,
    /// Amount of polygons
    pub poly_count: i32,
    /// Minima of all verticies
    pub mins: [f32; 3],
    /// Maxima of all verticies
    pub maxs: [f32; 3],
    pub area: i32,
}

impl BspParseable for OccluderData {
    fn from_reader(data: &mut LumpReader) -> Self {
        Self {
            flags: data.read_i32(),
            first_poly: data.read_i32(),
            poly_count: data.read_i32(),
            mins: [data.read_f32(), data.read_f32(), data.read_f32()],
            maxs: [data.read_f32(), data.read_f32(), data.read_f32()],
            area: data.read_i32(),
        }
    }
}

#[derive(Debug)]
pub struct OccluderPolyData {
    /// Index into occluder vertex indicies
    pub first_vertex_index: i32,
    /// amount of vertex indicies
    pub vertex_count: i32,
    /// The plane number
    pub plane_num: i32,
}

impl BspParseable for OccluderPolyData {
    fn from_reader(data: &mut LumpReader) -> Self {
        Self {
            first_vertex_index: data.read_i32(),
            vertex_count: data.read_i32(),
            plane_num: data.read_i32(),
        }
    }
}

#[derive(Debug)]
pub struct Occluder {
    pub count: i32,
    pub occluder_data: Vec<OccluderData>,
    pub poly_data_count: i32,
    pub poly_data: Vec<OccluderPolyData>,
    pub vertex_index_count: i32,
    pub vertex_indicies: Vec<i32>,
}

impl BspParseable for Occluder {
    fn from_reader(data: &mut LumpReader) -> Self {
        let count = data.read_i32();
        let mut occluder_data = vec![];

        for _ in 0..count {
            occluder_data.push(OccluderData::from_reader(data));
        }
        let poly_data_count = data.read_i32();
        let mut poly_data = vec![];

        for _ in 0..poly_data_count {
            poly_data.push(OccluderPolyData::from_reader(data));
        }
        let vertex_index_count = data.read_i32();
        let mut vertex_indicies = vec![];

        for _ in 0..vertex_index_count {
            vertex_indicies.push(data.read_i32())
        }

        Occluder {
            count,
            occluder_data,
            poly_data_count,
            poly_data,
            vertex_index_count,
            vertex_indicies,
        }
    }
}

#[derive(Debug)]
pub struct Edge {
    /// Vertex indicies
    vertex_indicies: [u16; 2],
}

impl BspParseable for Edge {
    fn from_reader(data: &mut LumpReader) -> Self {
        Self {
            vertex_indicies: [data.read_u16(), data.read_u16()],
        }
    }
}

#[derive(Debug)]
pub struct Model {
    /// Bounding box
    mins: Vector,
    /// Bounding box
    maxs: Vector,
    /// For sounds/lights
    origin: Vector,
    /// Index into node array
    head_node: i32,
    /// Infex into face array
    first_face: i32,
    /// Number of faces
    num_faces: i32,
}

impl BspParseable for Model {
    fn from_reader(data: &mut LumpReader) -> Self {
        Self {
            mins: (data.read_f32(), data.read_f32(), data.read_f32()),
            maxs: (data.read_f32(), data.read_f32(), data.read_f32()),
            origin: (data.read_f32(), data.read_f32(), data.read_f32()),
            head_node: data.read_i32(),
            first_face: data.read_i32(),
            num_faces: data.read_i32(),
        }
    }
}

#[derive(Debug)]
pub struct Brush {
    /// First brushside
    first_side: i32,
    /// Number of brushsides
    num_sides: i32,
    /// Contents flags
    contents: i32,
}

impl BspParseable for Brush {
    fn from_reader(data: &mut LumpReader) -> Self {
        Self {
            first_side: data.read_i32(),
            num_sides: data.read_i32(),
            contents: data.read_i32(),
        }
    }
}

#[derive(Debug)]
pub struct Brushside {
    /// Facing out of leaf
    plane_num: u16,
    /// Texture info
    texinfo: i16,
    /// Displacement info
    dispinfo: i16,
    /// Is the side a bevel plane?
    bevel: i16,
}

impl BspParseable for Brushside {
    fn from_reader(data: &mut LumpReader) -> Self {
        Self {
            plane_num: data.read_u16(),
            texinfo: data.read_i16(),
            dispinfo: data.read_i16(),
            bevel: data.read_i16(),
        }
    }
}

#[derive(Debug)]
pub struct Area {
    num_area_portals: i32,
    first_area_portal: i32,
}

impl BspParseable for Area {
    fn from_reader(data: &mut LumpReader) -> Self {
        Self {
            num_area_portals: data.read_i32(),
            first_area_portal: data.read_i32(),
        }
    }
}

#[derive(Debug)]
pub struct AreaPortal {
    portal_key: u16,
    other_area: u16,
    first_clip_portal_vert: u16,
    num_clip_portal_verts: u16,
    plane_num: i32,
}

impl BspParseable for AreaPortal {
    fn from_reader(data: &mut LumpReader) -> Self {
        Self {
            portal_key: data.read_u16(),
            other_area: data.read_u16(),
            first_clip_portal_vert: data.read_u16(),
            num_clip_portal_verts: data.read_u16(),
            plane_num: data.read_i32(),
        }
    }
}

#[derive(Debug)]
pub struct CDispSubNeighbor {
    neighbor_index: u16,
    neighbor_orientation: u8,
    span: u8,
    neighbor_span: u8,
}

impl BspParseable for CDispSubNeighbor {
    fn from_reader(data: &mut LumpReader) -> Self {
        let neighbor_index = data.read_u16();
        let neighbor_orientation = data.read_u8();
        let span = data.read_u8();
        let neighbor_span = data.read_u8();
        //println!("N_SPAN: {} : {}", neighbor_span, neighbor_span == 0);
        Self {
            neighbor_index,
            neighbor_orientation,
            span,
            neighbor_span,
        }
    }
}

#[derive(Debug)]
pub struct CDispNeighbor {
    sub_neighbors: Vec<CDispSubNeighbor>,
}

impl BspParseable for CDispNeighbor {
    fn from_reader(data: &mut LumpReader) -> Self {
        let mut out = Self {
            sub_neighbors: vec![],
        };
        for i in 0..2 {
            let neighbor = CDispSubNeighbor::from_reader(data);
            let is_last = neighbor.neighbor_span == 0;
            println!("{} : {}", i, is_last);
            out.sub_neighbors.push(neighbor);
            if is_last && i == 0 {
                println!("WOULD BREAK");
                //break;
            }
        }
        out
    }
}

#[derive(Debug)]
pub struct CDispCornerNeighbors {
    neighbors: [u16; 4],
    num_neighbors: u8,
}

impl BspParseable for CDispCornerNeighbors {
    fn from_reader(data: &mut LumpReader) -> Self {
        Self {
            neighbors: [
                data.read_u16(),
                data.read_u16(),
                data.read_u16(),
                data.read_u16(),
            ],
            num_neighbors: data.read_u8(),
        }
    }
}

#[derive(Debug)]
pub struct DisplacementInfo {
    start_position: Vector,
    disp_vert_start: i32,
    disp_tri_start: i32,
    power: i32,
    min_tesselation: i32,
    smoothing_angle: f32,
    contents: i32,
    map_face: u16,
    lightmap_alpha_start: i32,
    lightmap_sample_position_start: i32,
    neighbor_data: (), // Temporary padding (90 bytes because I am lost)
    allowed_verts: Vec<u32>,
}

impl BspParseable for DisplacementInfo {
    fn from_reader(data: &mut LumpReader) -> Self {
        println!("{}:{}", data.get_pos(), data.get_len());
        Self {
            start_position: Vector::from_reader(data),
            disp_vert_start: data.read_i32(),
            disp_tri_start: data.read_i32(),
            power: data.read_i32(),
            min_tesselation: data.read_i32(),
            smoothing_angle: data.read_f32(),
            contents: data.read_i32(),
            map_face: data.read_u16(),
            lightmap_alpha_start: data.read_i32(),
            lightmap_sample_position_start: data.read_i32(),
            neighbor_data: data.skip_bytes(90), // Skips the neighbor section because my readers are broken.
            allowed_verts: data.read_x_u32(10),
        }
    }
}
