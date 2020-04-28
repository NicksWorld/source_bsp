use std::collections::HashMap;
use std::convert::TryInto;

use regex::Regex;

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
pub struct ParsedLumps {
    entities: Vec<HashMap<String, String>>,
    planes: Vec<Plane>,
    texdata: Vec<TexData>,
    vertex_list: Vec<Vertex>,
    texinfo: Vec<TexInfo>,
    faces: Vec<Face>,
    lightmap_samples: Vec<LightMapSample>,
}

pub fn parse_lump_data(lumps: Vec<Lump>, full_data: &[u8]) -> ParsedLumps {
    let mut parsed = ParsedLumps {
        entities: vec![],
        planes: vec![],
        texdata: vec![],
        vertex_list: vec![],
        texinfo: vec![],
        faces: vec![],
        lightmap_samples: vec![],
    };

    for (i, lump) in lumps.iter().enumerate() {
        let mut data = LumpReader::new(
            &full_data[lump.fileofs as usize..(lump.fileofs + lump.filelen) as usize],
        );

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
                let num_lumps = (lump.filelen) / (4 + 4 + 4 + 4 + 4);

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
                let num_lumps = (lump.filelen) / 32;

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
                let num_lumps = (lump.filelen) / 12;

                for _ in 0..num_lumps {
                    parsed.vertex_list.push(Vertex {
                        x: data.read_f32(),
                        y: data.read_f32(),
                        z: data.read_f32(),
                    });
                }
            }
            i if i == LumpType::Visibility as usize => (), // This one will be a challenge
            i if i == LumpType::Nodes as usize => (),      // This one also will be a challenge
            i if i == LumpType::Texinfo as usize => {
                let num_lumps = (lump.filelen) / 72;

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
                let num_lumps = (lump.filelen) / 56;

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
                let num_lumps = (lump.filelen) / 4;

                for _ in 0..num_lumps {
                    parsed.lightmap_samples.push(LightMapSample {
                        r: data.read_u8(),
                        g: data.read_u8(),
                        b: data.read_u8(),
                        exponent: data.read_i8(),
                    });
                }
            }
            i if i == LumpType::Occlusion as usize => (),
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
