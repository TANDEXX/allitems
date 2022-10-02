#!/bin/nano
// you can edit this shitty code if you want

pub mod tables;
pub mod gen_java;
pub mod gen_tex;
pub mod util;

use std::fs::{File};
use std::io::{Read, Write};

fn main() {
	println!("loading needed files...");
	let mut material_table_file = String::new();
	let mut item_type_table_file = String::new();
	File::open("tables/materials.txt").unwrap().read_to_string(&mut material_table_file).unwrap();
	File::open("tables/item types.txt").unwrap().read_to_string(&mut item_type_table_file).unwrap();

	println!("loading tables...");
	let material_table = tables::read_material_table(&material_table_file);
	let item_type_table = tables::read_item_type_table(&item_type_table_file);

	println!("generating {} items...", material_table.len() * item_type_table.len());
	let mut en_us_lang_file = String::new();
	en_us_lang_file.push_str("{\"itemGroup.allitems_creative_tab\":\"All Items\"");
	gen_java::gen_items(&material_table, &item_type_table, &mut en_us_lang_file);

	println!("generating textures...");
	gen_tex::gen(&material_table, &item_type_table);

	en_us_lang_file.push_str("}\n");
	println!("saving lang file...");
	File::create("../src/main/resources/assets/allitems/lang/en_us.json").unwrap().write(en_us_lang_file.as_bytes()).unwrap();

}

pub fn crash(msg: &str) -> ! {

	println!("crashed for being fuk or \"{}\"", msg);
	std::process::exit(1)
}
