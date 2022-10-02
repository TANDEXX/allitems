#!/bin/nano

use std::fs::{File};
use std::io::{BufWriter, Write, stdout};
use png::HasParameters;
//use crate::crash;
use crate::tables::{Material, ItemType};
use crate::util::{item_name};

pub fn gen(material_table: &Vec<Material>, item_type_table: &Vec<ItemType>) {
	let mut current_item = 0;

	for item_type in item_type_table {
		let mut double_layer_tex = false;
		let mut tex_file1 = File::open("src/main.rs").unwrap();
		let tex_file = match File::open(format!("raw textures/{}.png", item_type.name)) {

			Ok(result) => result,
			Err(_) => {
				double_layer_tex = true;
				tex_file1 = File::open(format!("raw textures/{}1.png", item_type.name)).unwrap();
				File::open(format!("raw textures/{}0.png", item_type.name)).unwrap()
			}

		};
		let tex_decoder = png::Decoder::new(tex_file);
		let (tex_info, mut tex_reader) = tex_decoder.read_info().unwrap();
		let mut tex = vec![0; tex_info.buffer_size()];
		let mut tex1 = vec![0; tex_info.buffer_size()];
		//if tex.len() != 16 * 16 {crash(format!("texture not 16x16 ({})", item_type.name).as_str())}
		tex_reader.next_frame(&mut tex).unwrap();

		if double_layer_tex {

			let tex1_decoder = png::Decoder::new(tex_file1);
			let (_tex1_info, mut tex1_reader) = tex1_decoder.read_info().unwrap();
			tex1_reader.next_frame(&mut tex1).unwrap();

		} else {
			tex1 = tex.clone();
		}

		for material in material_table {
			if current_item % 120 == 0 {
				print!("\r{}%", current_item as f32 / (material_table.len() * item_type_table.len()) as f32 * 100.0);
				stdout().flush().unwrap();
			}
			let mut new_tex = vec![0; tex.len()];
			let mut no_idea = 0u8;
			for a in 0..tex.len() {

				new_tex[a] = tex[a].clone();

				if no_idea == 0 {if tex[a+3] == 0xff {new_tex[a] = (tex[a] as f64 / (255.0 / (material.main_color >> 16 & 0xff) as f64)) as u8;} else {new_tex[a] = (tex1[a] as f64 / (255.0 / (material.secondary_color >> 16 & 0xff) as f64)) as u8;}}
				if no_idea == 1 {if tex[a+2] == 0xff {new_tex[a] = (tex[a] as f64 / (255.0 / (material.main_color >> 8 & 0xff) as f64)) as u8;} else {new_tex[a] = (tex1[a] as f64 / (255.0 / (material.secondary_color >> 8 & 0xff) as f64)) as u8;}}
				if no_idea == 2 {if tex[a+1] == 0xff {new_tex[a] = (tex[a] as f64 / (255.0 / (material.main_color & 0xff) as f64)) as u8;} else {new_tex[a] = (tex1[a] as f64 / (255.0 / (material.secondary_color & 0xff) as f64)) as u8;}}
				if no_idea == 3 {if tex[a] != 0xff {new_tex[a] = tex1[a];}}
				//if no_idea == 2 {new_tex[a] = (-(new_tex[a] as i32) + (material.main_color as i32 >> 24 & 0xff)) as u8; if new_tex[a] > tex[a] {new_tex[a] = 0x00;}new_tex[a] = (-(new_tex[a] as i8) as u8);}

				no_idea+=1;
				if no_idea == 4 {no_idea = 0;}

			}
			let new_tex_file = File::create(format!("../src/main/resources/assets/allitems/textures/item/{}.png", item_name(&material, &item_type)).as_str()).unwrap();
			let ref mut new_tex_buf_writer = BufWriter::new(new_tex_file);
			let mut new_tex_encoder = png::Encoder::new(new_tex_buf_writer, 16, 16);
			new_tex_encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
			let mut new_tex_writer = new_tex_encoder.write_header().unwrap();
			new_tex_writer.write_image_data(&new_tex).unwrap();

			let mut model_file = File::create(format!("../src/main/resources/assets/allitems/models/item/{}.json", item_name(&material, &item_type)).as_str()).unwrap();
			model_file.write(format!("{o}\"parent\":\"item/generated\",\"textures\":{o}\"layer0\":\"allitems:item/{item_name}\"{c}{c}", o = "{", c = "}", item_name = item_name(&material, &item_type)).as_bytes()).unwrap();


			current_item+=1;
		}

	}

	print!("\r");
}
