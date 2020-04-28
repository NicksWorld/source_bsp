#[macro_use]
extern crate lazy_static;

use std::fs::File;
use std::io::Read;

use std::convert::TryInto;

mod lumps;
use lumps::*;

#[derive(Debug)]
struct BspHeader {
    ident: i32,
    version: i32,
}

struct BspParser {
    pub data: Vec<u8>,
}

impl BspParser {
    pub fn new(path: &str) -> Result<Self, std::io::Error> {
        let mut file = File::open(path)?;
        let mut contents = vec![];

        file.read_to_end(&mut contents)?;

        Ok(BspParser { data: contents })
    }

    pub fn fetch_header(&self) -> BspHeader {
        BspHeader {
            ident: i32::from_le_bytes(self.data[0..4].try_into().unwrap()),
            version: i32::from_le_bytes(self.data[4..8].try_into().unwrap()),
        }
    }

    pub fn read_lump_info(&self) -> Vec<Lump> {
        let mut lumps = vec![];

        for i in 0..64 {
            println!("{}", i);
            lumps.push(Lump {
                fileofs: i32::from_le_bytes(
                    self.data[(8 + (i * 16))..(8 + 4 + (i * 16))]
                        .try_into()
                        .unwrap(),
                ),
                filelen: i32::from_le_bytes(
                    self.data[(8 + 4 + (i * 16))..(8 + 4 + 4 + (i * 16))]
                        .try_into()
                        .unwrap(),
                ),
                version: i32::from_le_bytes(
                    self.data[(8 + 8 + (i * 16))..(8 + 4 + 8 + (i * 16))]
                        .try_into()
                        .unwrap(),
                ),
                ident: self.data[(8 + 12 + (i * 16))..(8 + 4 + 12 + (i * 16))]
                    .try_into()
                    .unwrap(),
            })
        }

        lumps
    }
}

#[test]
fn test_program() {
    let bsp_parser = BspParser::new("arena_badlands.bsp").unwrap();

    println!("{:?}", bsp_parser.fetch_header());
    println!("{:?}", bsp_parser.read_lump_info());

    let lumps = bsp_parser.read_lump_info();

    println!("{:#?}", parse_lump_data(lumps, &bsp_parser.data));
}
