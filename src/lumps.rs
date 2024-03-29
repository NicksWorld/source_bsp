use std::collections::HashMap;
use std::convert::TryInto;
#[derive(Debug)]
pub struct Lump {
    pub fileofs: i32,
    pub filelen: i32,
    pub version: i32,
    pub ident: [u8; 4],
}

pub enum LumpType {
    Entities = 0,
    Plane = 1,
    Texdata = 2,
    Vertexes = 3,
    Visibility = 4,
    Nodes = 5,
    Texinfo = 6,
    Faces = 7,
    Lighting = 8,
    Occlusion = 9,
    Leafs = 10,
    Faceids = 11,
    Edges = 12,
    Surfedges = 13,
    Models = 14,
    Worldlights = 15,
    Leaffaces = 16,
    Leafbrushes = 17,
    Brushes = 18,
    Brushsides = 19,
    Areas = 20,
    Areaportals = 21,
    UNUSED0 = 22,
    UNUSED1 = 23,
    UNUSED2 = 24,
    UNUSED3 = 25,
    Dispinfo = 26,
    Originalfaces = 27,
    Physdisp = 28,
    Physcollide = 29,
    Vertnormals = 30,
    Vertnormalindices = 31,
    DispLightmapAlphas = 32,
    DispVerts = 33,
    DispLightmapSamplePositions = 34,
    GameLump = 35,
    Leafwaterdata = 36,
    Primitives = 37,
    Primverts = 38,
    Primindicies = 39,
    Pakfile = 40,
    Clipportalverts = 41,
    Cubemaps = 42,
    TexdataStringData = 43,
    TexdataStringTable = 44,
    Overlays = 45,
    Leafmindisttowater = 46,
    FaceMacroTextureInfo = 47,
    DispTris = 48,
    Physcollidesurface = 49,
    Wateroverlays = 50,
    LeafAmbientIndexHDR = 51,
    LeafAmbientIndex = 52,
    LightingHDR = 53,
    WorldlightsHDR = 54,
    LeafAmbientLightingHDR = 55,
    LeafAmbientLighting = 56,
    Xzippakfile = 57,
    FacesHDR = 58,
    MapFlags = 59,
    OverlayFades = 60,
    OverlaySystemLevels = 61,
    Physlevel = 62,
    DispMultiblend = 63,
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

    pub fn read_x_u32(&mut self, count: usize) -> Vec<u32> {
        let mut out = vec![];
        for _ in 0..count {
            out.push(self.read_u32());
        }
        out
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

    pub fn skip_bytes(&mut self, byte_count: usize) {
        self.position += byte_count;
    }

    pub fn get_len(&self) -> usize {
        self.data.len()
    }
}

pub mod lump_types;
use lump_types::*;

pub mod LumpParser {
    use crate::lumps::*;
    use regex::Regex;

    #[derive(Debug, Default)]
    pub struct ParsedLumps {
        pub entities: Vec<Entity>,
        pub planes: Vec<Plane>,
        pub texdata: Vec<TexData>,
        pub vertex_list: Vec<Vertex>,
        pub nodes: Vec<Node>,
        pub texinfo: Vec<TexInfo>,
        pub faces: Vec<Face>,
        pub lightmap_samples: Vec<LightmapSample>,
        pub occluders: Vec<Occluder>,
        pub edges: Vec<Edge>,
        pub surfedges: Vec<i32>,
        pub models: Vec<Model>,
        pub leaf_faces: Vec<u16>,
        pub leaf_brushes: Vec<u16>,
        pub brushes: Vec<Brush>,
        pub brushsides: Vec<Brushside>,
        pub areas: Vec<Area>,
        pub area_portals: Vec<AreaPortal>,
        pub displacement_info: Vec<DisplacementInfo>,
        pub original_faces: Vec<Face>,
        pub physics_models: Vec<PhysicsModel>,
    }

    macro_rules! parse_type {
        ($data:expr, $dst:expr, $kind:ty) => {{
            while $data.get_pos() < $data.get_len() {
                // Pushes the data read to the destination
                $dst.push(<$kind>::from_reader(&mut $data));
            }
        }};
    }

    fn decompress_lumps(mut data: LumpReader) -> LumpReader {
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

        let data_in = [
            &properties as &[u8],
            &(actual_size as u64).to_le_bytes(),
            &data.get_data()[17..],
        ]
        .concat();

        lzma_rs::lzma_decompress(&mut std::io::Cursor::new(data_in), &mut out).unwrap();

        LumpReader::new(&out)
    }

    pub fn parse_lump_data(lumps: Vec<Lump>, full_data: &[u8]) -> ParsedLumps {
        // Creates ParsedLumps empty and ready to be filled.
        let mut parsed: ParsedLumps = Default::default();

        for (i, lump) in lumps.iter().enumerate() {
            if lump.fileofs == 0 {
                continue; // Lump isn't actually included
            }

            let mut data = LumpReader::new(
                &full_data[lump.fileofs as usize..(lump.fileofs + lump.filelen) as usize],
            );

            if lump.ident != [0, 0, 0, 0] {
                // The packet is compressed. Read the header, convert to normal LZMA and decompress
                data = decompress_lumps(data);
            }

            match i {
                i if i == LumpType::Entities as usize => {
                    lazy_static! {
                    // Regex to match the contents within currly brackets
                                static ref BLOCK_RE: Regex = Regex::new("(\\{(?:.|\n)+?\\})").unwrap();
                    // Regex to get the key:value pairs out of the data
                                static ref ITEM_RE: Regex = Regex::new("\"(.+?)\" \"(.+?)\"\n").unwrap();
                            }

                    let data = String::from_utf8_lossy(data.get_data());

                    for capture in BLOCK_RE.captures_iter(&data) {
                        let mut map = HashMap::new();

                        // Loop through individual entity sections
                        for capture in ITEM_RE.captures_iter(&capture[0]) {
                            // Take the key:value pair and put it in a hashmap
                            map.insert(capture[1].to_string(), capture[2].to_string());
                        }
                        parsed.entities.push(map);
                    }
                }
                i if i == LumpType::Plane as usize => parse_type!(data, parsed.planes, Plane),
                i if i == LumpType::Texdata as usize => parse_type!(data, parsed.texdata, TexData),
                i if i == LumpType::Vertexes as usize => {
                    parse_type!(data, parsed.vertex_list, Vertex)
                }
                i if i == LumpType::Visibility as usize => (), // This one will be a challenge
                i if i == LumpType::Nodes as usize => parse_type!(data, parsed.nodes, Node),
                i if i == LumpType::Texinfo as usize => parse_type!(data, parsed.texinfo, TexInfo),
                i if i == LumpType::Faces as usize => parse_type!(data, parsed.faces, Face),
                i if i == LumpType::Lighting as usize => {
                    parse_type!(data, parsed.lightmap_samples, LightmapSample)
                }
                i if i == LumpType::Occlusion as usize => {
                    parse_type!(data, parsed.occluders, Occluder)
                }
                i if i == LumpType::Leafs as usize => (),
                i if i == LumpType::Faceids as usize => (),
                i if i == LumpType::Edges as usize => parse_type!(data, parsed.edges, Edge),
                i if i == LumpType::Surfedges as usize => {
                    while data.get_pos() < data.get_len() {
                        parsed.surfedges.push(data.read_i32())
                    }
                }
                i if i == LumpType::Models as usize => parse_type!(data, parsed.models, Model),
                i if i == LumpType::Worldlights as usize => (),
                i if i == LumpType::Leaffaces as usize => {
                    while data.get_pos() < data.get_len() {
                        parsed.leaf_faces.push(data.read_u16())
                    }
                }
                i if i == LumpType::Leafbrushes as usize => {
                    while data.get_pos() < data.get_len() {
                        parsed.leaf_brushes.push(data.read_u16())
                    }
                }
                i if i == LumpType::Brushes as usize => parse_type!(data, parsed.brushes, Brush),
                i if i == LumpType::Brushsides as usize => {
                    parse_type!(data, parsed.brushsides, Brushside)
                }
                i if i == LumpType::Areas as usize => parse_type!(data, parsed.areas, Area),
                i if i == LumpType::Areaportals as usize => {
                    parse_type!(data, parsed.area_portals, AreaPortal)
                }
                i if i == LumpType::UNUSED0 as usize => (),
                i if i == LumpType::UNUSED1 as usize => (),
                i if i == LumpType::UNUSED2 as usize => (),
                i if i == LumpType::UNUSED3 as usize => (),
                i if i == LumpType::Dispinfo as usize => {
                    parse_type!(data, parsed.displacement_info, DisplacementInfo)
                }
                i if i == LumpType::Originalfaces as usize => {
                    parse_type!(data, parsed.original_faces, Face)
                }
                i if i == LumpType::Physdisp as usize => (), // Needs work on finding the structure
                i if i == LumpType::Physcollide as usize => {
                    parse_type!(data, parsed.physics_models, PhysicsModel)
                }
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
                i if i == LumpType::Wateroverlays as usize => (),
                i if i == LumpType::LeafAmbientIndexHDR as usize => (),
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
}
