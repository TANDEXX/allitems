#!/bin/nano

use crate::crash;
use crate::util::{hex_basic_color};

pub struct Material<'a> {

	pub name: &'a str,
	pub en_us_name: &'a str,
	pub main_color: u32,
	pub secondary_color: u32,
	pub tier: u16,
	pub special: MaterialSpecial,

}

pub struct ItemType<'a> {

	pub name: &'a str,
	pub en_us_name: &'a str,
	pub special: ItemTypeSpecial,

}

pub enum MaterialSpecial {

	No,

}

pub enum ItemTypeSpecial {

	No,
	Fruit,

}

pub fn read_material_table<'a>(table: &'a String) -> Vec<Material<'a>> {
	let mut result = Vec::new();
	let mut last_enter = -1isize;
	let mut str_array = [""; 7];
	let mut str_array_addr = 0usize;

	for a in 0..table.len() {

		if &table[a..a+1] == "\n" {

			str_array[str_array_addr.clone()] = &table[(last_enter+1) as usize..a];
			str_array_addr+=1;
			if str_array_addr == str_array.len() {

				result.push(Material {
					name: str_array[0],
					en_us_name: str_array[1],
					main_color: hex_basic_color(str_array[2]),
					secondary_color: hex_basic_color(str_array[3]),
					tier: str_array[4].to_string().parse::<u16>().unwrap(),
					special: if str_array[5] == "no" {
						MaterialSpecial::No
					} else {
						crash(format!("material has completly wronk special, material is \"{}\"", str_array[0]).as_str())
					},
				});
				str_array_addr = 0;
				//println!("{}", result[result.len() - 1].main_color);
				//println!("{}", str_array[2]);

			}

			last_enter = a as isize;

		}

	}

	result
}

pub fn read_item_type_table<'a>(table: &'a String) -> Vec<ItemType<'a>> {
	let mut result = Vec::new();
	let mut last_enter = -1isize;
	let mut str_array = [""; 4];
	let mut str_array_addr = 0usize;

	for a in 0..table.len() {

		if &table[a..a+1] == "\n" {

			str_array[str_array_addr.clone()] = &table[(last_enter+1) as usize..a];
			str_array_addr+=1;
			if str_array_addr == str_array.len() {

				result.push(ItemType {
					name: str_array[0],
					en_us_name: str_array[1],
					special: if str_array[2] == "no" {
						ItemTypeSpecial::No
					} else if str_array[2] == "fruit" {
						ItemTypeSpecial::Fruit
					} else {
						crash(format!("item type has wronk special, item type is \"{}\"", str_array[0]).as_str())
					},
				});
				//println!("{}", str_array[0]);
				str_array_addr = 0;

			}

			last_enter = a as isize;

		}

	}

	result
}
