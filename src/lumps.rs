use std::collections::HashMap;
use std::convert::TryInto;

use regex::Regex;

use std::io::Write;

#[derive(Debug)]
pub struct Lump {
    pub fileofs: i32,
    pub filelen: i32,
    pub version: i32,
    pub ident: [u8; 4],
}

pub enum LumpType {
    Entities = 0,
    Plane,
    Texdata,
    Vertexes,
    Visibility,
    Nodes,
    Texinfo,
    Faces,
    Lighting,
    Occlusion,
    Leafs,
    Faceids,
    Edges,
    Surfedges,
    Models,
    Worldlights,
    Leaffaces,
    Leafbrushes,
    Brushes,
    Brushsides,
    Areas,
    Areaportals,
    Portals,
    UNUSED0,
    UNUSED1,
    UNUSED2,
    UNUSED3,
    Dispinfo,
    Originalfaces,
    Physdisp,
    Physcollide,
    Vertnormals,
    Vertnormalindices,
    DispLightmapAlphas,
    DispVerts,
    DispLightmapSamplePositions,
    GameLump,
    Leafwaterdata,
    Primitives,
    Primverts,
    Primindicies,
    Pakfile,
    Clipportalverts,
    Cubemaps,
    TexdataStringData,
    TexdataStringTable,
    Overlays,
    Leafmindisttowater,
    FaceMacroTextureInfo,
    DispTris,
    Physcollidesurface,
    PropBlob,
    Wateroverlays,
    Lightmappages,
    LeafAmbientIndexHDR,
    Lightmappageinfos,
    LeafAmbientIndex,
    LightingHDR,
    WorldlightsHDR,
    LeafAmbientLightingHDR,
    LeafAmbientLighting,
    Xzippakfile,
    FacesHDR,
    MapFlags,
    OverlayFades,
    OverlaySystemLevels,
    Physlevel,
    DispMultiblend,
}

pub struct LumpReader {
    position: usize,
    data: Vec<u8>,
}

impl LumpReader {
    pub fn new(data: &[u8]) -> Self {
        LumpReader {
            position: 0,
            data: data.to_vec(),
        }
    }

    pub fn read_f32(&mut self) -> f32 {
        self.position += 4;
        f32::from_le_bytes(
            self.data[self.position - 4..self.position]
                .try_into()
                .unwrap(),
        )
    }

    pub fn read_i32(&mut self) -> i32 {
        self.position += 4;
        i32::from_le_bytes(
            self.data[self.position - 4..self.position]
                .try_into()
                .unwrap(),
        )
    }

    pub fn read_u32(&mut self) -> u32 {
        self.position += 4;
        u32::from_le_bytes(
            self.data[self.position - 4..self.position]
                .try_into()
                .unwrap(),
        )
    }

    pub fn read_u16(&mut self) -> u16 {
        self.position += 2;
        u16::from_le_bytes(
            self.data[self.position - 2..self.position]
                .try_into()
                .unwrap(),
        )
    }

    pub fn read_i16(&mut self) -> i16 {
        self.position += 2;
        i16::from_le_bytes(
            self.data[self.position - 2..self.position]
                .try_into()
                .unwrap(),
        )
    }

    pub fn read_u8(&mut self) -> u8 {
        self.position += 1;
        u8::from_le_bytes(
            self.data[self.position - 1..self.position]
                .try_into()
                .unwrap(),
        )
    }

    pub fn read_i8(&mut self) -> i8 {
        self.position += 1;
        i8::from_le_bytes(
            self.data[self.position - 1..self.position]
                .try_into()
                .unwrap(),
        )
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    pub fn get_pos(&self) -> usize {
        self.position
    }

    pub fn get_len(&self) -> usize {
        self.data.len()
    }
}

#[derive(Debug)]
pub struct Plane {
    vec_x: f32,
    vec_y: f32,
    vec_z: f32,

    dist_from_origin: f32,
    r#type: i32,
}

#[derive(Debug)]
pub struct TexData {
    ref_r: f32,
    ref_g: f32,
    ref_b: f32,

    texdata_string_table_index: i32,
    width: i32,
    height: i32,
    view_width: i32,
    view_height: i32,
}

#[derive(Debug)]
pub struct Vertex {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Debug)]
pub struct Node {
    plane_num: i32,
    children: [i32; 2],
    mins: [i16; 3],
    maxs: [i16; 3],
    first_face: u16,
    num_faces: u16,
    area: i16,
    padding: i16,
}

#[derive(Debug)]
pub struct TexInfo {
    texture_vecs: [[f32; 4]; 2],
    lightmap_vecs: [[f32; 4]; 2],
    flags: i32,
    tex_data: i32,
}

#[derive(Debug)]
pub struct Face {
    plane_num: u16,
    side: u8,
    on_node: u8,
    first_edge: i32,
    num_edges: i16,
    texinfo: i16,
    displacement_info: i16,
    surface_fog_volume_id: i16,
    styles: [u8; 4],
    light_offset: i32,
    area: f32,
    lightmap_texture_mins_in_luxels: [i32; 2],
    lightmap_texture_size_in_luxels: [i32; 2],
    original_face: i32,
    num_primitives: u16,
    first_primitave_id: u16,
    smoothing_groups: u32,
}

#[derive(Debug)]
pub struct LightMapSample {
    r: u8,
    g: u8,
    b: u8,
    exponent: i8,
}

#[derive(Debug)]
pub struct OccluderData {
    flags: i32,
    first_poly: i32,
    poly_count: i32,
    mins: [f32; 3],
    maxs: [f32; 3],
    area: i32,
}

#[derive(Debug)]
pub struct OccluderPolyData {
    first_vertex_index: i32,
    vertex_count: i32,
    plane_num: i32,
}

#[derive(Debug)]
pub struct Occluder {
    count: i32,
    occluder_data: Vec<OccluderData>,
    poly_data_count: i32,
    poly_data: Vec<OccluderPolyData>,
    vertex_index_count: i32,
    vertex_indicies: Vec<i32>,
}

#[derive(Debug)]
pub struct ParsedLumps {
    pub entities: Vec<HashMap<String, String>>,
    pub planes: Vec<Plane>,
    pub texdata: Vec<TexData>,
    pub vertex_list: Vec<Vertex>,
    pub nodes: Vec<Node>,
    pub texinfo: Vec<TexInfo>,
    pub faces: Vec<Face>,
    pub lightmap_samples: Vec<LightMapSample>,
    pub occluders: Vec<Occluder>,
}

pub fn parse_lump_data(lumps: Vec<Lump>, full_data: &[u8]) -> ParsedLumps {
    let mut parsed = ParsedLumps {
        entities: vec![],
        planes: vec![],
        texdata: vec![],
        vertex_list: vec![],
        nodes: vec![],
        texinfo: vec![],
        faces: vec![],
        lightmap_samples: vec![],
        occluders: vec![],
    };

    for (i, lump) in lumps.iter().enumerate() {
        if lump.fileofs == 0 {
            continue; // Lump isn't actually included
        }

        let mut data = LumpReader::new(
            &full_data[lump.fileofs as usize..(lump.fileofs + lump.filelen) as usize],
        );

        if lump.ident != [0, 0, 0, 0] {
            // The packet is compressed. Read the header, convert to normal LZMA and decompress.
            let _ = data.read_u32(); // id
            let actual_size = data.read_u32();
            let _ = data.read_u32(); // lzma_size
            let properties = [
                data.read_u8(),
                data.read_u8(),
                data.read_u8(),
                data.read_u8(),
                data.read_u8(),
            ];

            let mut out = vec![0; actual_size as usize];

            let mut data_in = vec![];
            data_in.extend(&properties);
            data_in.extend(&(actual_size as u64).to_le_bytes());
            data_in.extend(&data.get_data()[17..]);

            lzma_rs::lzma_decompress(&mut std::io::Cursor::new(data_in), &mut out).unwrap();

            data = LumpReader::new(&out);
        }

        match i {
            i if i == LumpType::Entities as usize => {
                lazy_static! {
                    static ref BLOCK_RE: Regex = Regex::new("(\\{(?:.|\n)+?\\})").unwrap();
                    static ref ITEM_RE: Regex = Regex::new("\"(.+?)\" \"(.+?)\"\n").unwrap();
                }

                let data = String::from_utf8_lossy(data.get_data());

                for capture in BLOCK_RE.captures_iter(&data) {
                    let mut map = HashMap::new();

                    for capture in ITEM_RE.captures_iter(&capture[0]) {
                        map.insert(capture[1].to_string(), capture[2].to_string());
                    }
                    parsed.entities.push(map);
                }
            }
            i if i == LumpType::Plane as usize => {
                let num_lumps = (data.get_len()) / (4 + 4 + 4 + 4 + 4);

                for _ in 0..num_lumps {
                    parsed.planes.push(Plane {
                        vec_x: data.read_f32(),
                        vec_y: data.read_f32(),
                        vec_z: data.read_f32(),

                        dist_from_origin: data.read_f32(),
                        r#type: data.read_i32(),
                    });
                }
            }
            i if i == LumpType::Texdata as usize => {
                let num_lumps = (data.get_len()) / 32;

                for _ in 0..num_lumps {
                    parsed.texdata.push(TexData {
                        ref_r: data.read_f32(),
                        ref_g: data.read_f32(),
                        ref_b: data.read_f32(),

                        texdata_string_table_index: data.read_i32(),

                        width: data.read_i32(),
                        height: data.read_i32(),
                        view_width: data.read_i32(),
                        view_height: data.read_i32(),
                    });
                }
            }
            i if i == LumpType::Vertexes as usize => {
                let num_lumps = (data.get_len()) / 12;

                for _ in 0..num_lumps {
                    parsed.vertex_list.push(Vertex {
                        x: data.read_f32(),
                        y: data.read_f32(),
                        z: data.read_f32(),
                    });
                }
            }
            i if i == LumpType::Visibility as usize => (), // This one will be a challenge
            i if i == LumpType::Nodes as usize => {
                let num_lumps = (data.get_len()) / 32;

                for _ in 0..num_lumps {
                    parsed.nodes.push(Node {
                        plane_num: data.read_i32(),
                        children: [data.read_i32(), data.read_i32()],
                        mins: [data.read_i16(), data.read_i16(), data.read_i16()],
                        maxs: [data.read_i16(), data.read_i16(), data.read_i16()],
                        first_face: data.read_u16(),
                        num_faces: data.read_u16(),
                        area: data.read_i16(),
                        padding: data.read_i16(),
                    });
                }
            }
            i if i == LumpType::Texinfo as usize => {
                let num_lumps = (data.get_len()) / 72;

                for _ in 0..num_lumps {
                    parsed.texinfo.push(TexInfo {
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
                    });
                }
            }
            i if i == LumpType::Faces as usize => {
                let num_lumps = (data.get_len()) / 56;

                for _ in 0..num_lumps {
                    parsed.faces.push(Face {
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
                    });
                }
            }
            i if i == LumpType::Lighting as usize => {
                let num_lumps = (data.get_len()) / 4;

                for _ in 0..num_lumps {
                    parsed.lightmap_samples.push(LightMapSample {
                        r: data.read_u8(),
                        g: data.read_u8(),
                        b: data.read_u8(),
                        exponent: data.read_i8(),
                    });
                }
            }
            i if i == LumpType::Occlusion as usize => {
                while data.get_pos() < data.get_len() as usize {
                    let count = data.read_i32();
                    let mut occluder_data = vec![];

                    for _ in 0..count {
                        occluder_data.push(OccluderData {
                            flags: data.read_i32(),
                            first_poly: data.read_i32(),
                            poly_count: data.read_i32(),
                            mins: [data.read_f32(), data.read_f32(), data.read_f32()],
                            maxs: [data.read_f32(), data.read_f32(), data.read_f32()],
                            area: data.read_i32(),
                        });
                    }
                    let poly_data_count = data.read_i32();
                    let mut poly_data = vec![];

                    for _ in 0..poly_data_count {
                        poly_data.push(OccluderPolyData {
                            first_vertex_index: data.read_i32(),
                            vertex_count: data.read_i32(),
                            plane_num: data.read_i32(),
                        });
                    }
                    let vertex_index_count = data.read_i32();
                    let mut vertex_indicies = vec![];

                    for _ in 0..vertex_index_count {
                        vertex_indicies.push(data.read_i32())
                    }

                    parsed.occluders.push(Occluder {
                        count,
                        occluder_data,
                        poly_data_count,
                        poly_data,
                        vertex_index_count,
                        vertex_indicies,
                    });
                }
            }
            i if i == LumpType::Leafs as usize => (),
            i if i == LumpType::Faceids as usize => (),
            i if i == LumpType::Edges as usize => (),
            i if i == LumpType::Surfedges as usize => (),
            i if i == LumpType::Models as usize => (),
            i if i == LumpType::Worldlights as usize => (),
            i if i == LumpType::Leaffaces as usize => (),
            i if i == LumpType::Leafbrushes as usize => (),
            i if i == LumpType::Brushes as usize => (),
            i if i == LumpType::Brushsides as usize => (),
            i if i == LumpType::Areas as usize => (),
            i if i == LumpType::Areaportals as usize => (),
            i if i == LumpType::Portals as usize => (),
            i if i == LumpType::UNUSED0 as usize => (),
            i if i == LumpType::UNUSED1 as usize => (),
            i if i == LumpType::UNUSED2 as usize => (),
            i if i == LumpType::UNUSED3 as usize => (),
            i if i == LumpType::Dispinfo as usize => (),
            i if i == LumpType::Originalfaces as usize => (),
            i if i == LumpType::Physdisp as usize => (),
            i if i == LumpType::Physcollide as usize => (),
            i if i == LumpType::Vertnormals as usize => (),
            i if i == LumpType::Vertnormalindices as usize => (),
            i if i == LumpType::DispLightmapAlphas as usize => (),
            i if i == LumpType::DispVerts as usize => (),
            i if i == LumpType::DispLightmapSamplePositions as usize => (),
            i if i == LumpType::GameLump as usize => (),
            i if i == LumpType::Leafwaterdata as usize => (),
            i if i == LumpType::Primitives as usize => (),
            i if i == LumpType::Primverts as usize => (),
            i if i == LumpType::Primindicies as usize => (),
            i if i == LumpType::Pakfile as usize => (),
            i if i == LumpType::Clipportalverts as usize => (),
            i if i == LumpType::Cubemaps as usize => (),
            i if i == LumpType::TexdataStringData as usize => (),
            i if i == LumpType::TexdataStringTable as usize => (),
            i if i == LumpType::Overlays as usize => (),
            i if i == LumpType::Leafmindisttowater as usize => (),
            i if i == LumpType::FaceMacroTextureInfo as usize => (),
            i if i == LumpType::DispTris as usize => (),
            i if i == LumpType::Physcollidesurface as usize => (),
            i if i == LumpType::PropBlob as usize => (),
            i if i == LumpType::Wateroverlays as usize => (),
            i if i == LumpType::Lightmappages as usize => (),
            i if i == LumpType::LeafAmbientIndexHDR as usize => (),
            i if i == LumpType::Lightmappageinfos as usize => (),
            i if i == LumpType::LeafAmbientIndex as usize => (),
            i if i == LumpType::LightingHDR as usize => (),
            i if i == LumpType::WorldlightsHDR as usize => (),
            i if i == LumpType::LeafAmbientLightingHDR as usize => (),
            i if i == LumpType::LeafAmbientLighting as usize => (),
            i if i == LumpType::Xzippakfile as usize => (),
            i if i == LumpType::FacesHDR as usize => (),
            i if i == LumpType::MapFlags as usize => (),
            i if i == LumpType::OverlayFades as usize => (),
            i if i == LumpType::OverlaySystemLevels as usize => (),
            i if i == LumpType::Physlevel as usize => (),
            i if i == LumpType::DispMultiblend as usize => (),
            _ => (),
        }
    }

    parsed
}
