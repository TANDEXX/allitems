#!/bin/nano

use crate::tables::{Material, ItemType};

pub fn hex_digit(string: &str, index: usize) -> u8 {

	match &string[index..index+1] { // made shitty using laziness

		"1" => 1,
		"2" => 2,
		"3" => 3,
		"4" => 4,
		"5" => 5,
		"6" => 6,
		"7" => 7,
		"8" => 8,
		"9" => 9,
		"a"|"A"|"@" => 10,
		"b"|"B" => 11,
		"c"|"C" => 12,
		"d"|"D" => 13,
		"e"|"E" => 14,
		"f"|"F" => 15,
		_ => 0,

	}
}

pub fn hex_basic_color(string: &str) -> u32 {
	let mut result = hex_digit(string, 5) as u32;
	result |= (hex_digit(string, 4) as u32) << 4;
	result |= (hex_digit(string, 3) as u32) << 8;
	result |= (hex_digit(string, 2) as u32) << 12;
	result |= (hex_digit(string, 1) as u32) << 16;
	result |= (hex_digit(string, 0) as u32) << 20;
	result |= 0xff << 24;

	result
}

pub fn item_name(material: &Material, item_type: &ItemType) -> String {
	let mut result = String::new();
	result.push_str(material.name);
	result.push_str("_");
	result.push_str(item_type.name);

	result
}
