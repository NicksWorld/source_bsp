use crate::lumps::LumpReader;

pub trait BspParseable {
	fn from_reader(data: &mut LumpReader) -> Self;
}

#[derive(Debug)]
pub struct Plane {
	pub vec_x: f32,
	pub vec_y: f32,
	pub vec_z: f32,

	pub dist_from_origin: f32,
	pub r#type: i32,
}

impl BspParseable for Plane {
	fn from_reader(data: &mut LumpReader) -> Self {
		Self {
			vec_x: data.read_f32(),
			vec_y: data.read_f32(),
			vec_z: data.read_f32(),

			dist_from_origin: data.read_f32(),
			r#type: data.read_i32()
		}
	}
}

#[derive(Debug)]
pub struct TexData {
	pub ref_r: f32,
	pub ref_g: f32,
	pub ref_b: f32,

	pub texdata_string_table_index: i32,
	pub width: i32,
	pub height: i32,
	pub view_width: i32,
	pub view_height: i32,
}

impl BspParseable for TexData {
	fn from_reader(data: &mut LumpReader) -> Self {
		Self {
			ref_r: data.read_f32(),
			ref_g: data.read_f32(),
			ref_b: data.read_f32(),

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
	pub x: f32,
	pub y: f32,
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
	pub plane_num: i32,
	pub children: [i32; 2],
	pub mins: [i16; 3],
	pub maxs: [i16; 3],
	pub first_face: u16,
	pub num_faces: u16,
	pub area: i16,
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
	pub texture_vecs: [[f32; 4]; 2],
	pub lightmap_vecs: [[f32; 4]; 2],
	pub flags: i32,
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
	pub plane_num: u16,
	pub side: u8,
	pub on_node: u8,
	pub first_edge: i32,
	pub num_edges: i16,
	pub texinfo: i16,
	pub displacement_info: i16,
	pub surface_fog_volume_id: i16,
	pub styles: [u8; 4],
	pub light_offset: i32,
	pub area: f32,
	pub lightmap_texture_mins_in_luxels: [i32; 2],
	pub lightmap_texture_size_in_luxels: [i32; 2],
	pub original_face: i32,
	pub num_primitives: u16,
	pub first_primitave_id: u16,
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

#[derive(Debug)]
pub struct LightmapSample {
	pub r: u8,
	pub g: u8,
	pub b: u8,
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
	pub first_poly: i32,
	pub poly_count: i32,
	pub mins: [f32; 3],
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
	pub first_vertex_index: i32,
	pub vertex_count: i32,
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
