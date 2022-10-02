#!/bin/nano

use std::fs::{File};
use std::io::{Write};
use std::str::{FromStr};
use crate::tables::{Material, /*MaterialSpecial,*/ ItemType, ItemTypeSpecial};

const IMPORTS: &'static str = "package com.tandex.allitems.scriptGen;import com.tandex.allitems.*;import net.minecraft.world.food.FoodProperties;import net.minecraft.world.item.Item;import net.minecraftforge.eventbus.api.IEventBus;import net.minecraftforge.registries.*;";

pub fn gen_items(material_table: &Vec<Material>, item_type_table: &Vec<ItemType>, en_us_lang_file: &mut String) {
	let mut item_class = String::new();
	item_class.push_str(format!("{imports}public class Items0{o}public static void register(IEventBus a){o}items.register(a);{c}public static final DeferredRegister<Item> items=DeferredRegister.create(ForgeRegistries.ITEMS,\"allitems\");public static final RegistryObject[] itemArray0=new RegistryObject[]{o}", imports = IMPORTS, o = "{", c = "}").as_str());
	let mut item_class_array_idx = 0u16;
	let mut item_class_arrays = 0;

	for material in material_table {

		for item_type in item_type_table {
			let mut item_name = String::new();
			item_name.push_str(material.name);
			item_name.push_str("_");
			item_name.push_str(item_type.name);
			let mut item_en_us_name = String::new();
			item_en_us_name.push_str(material.en_us_name);
			item_en_us_name.push_str(" ");
			item_en_us_name.push_str(item_type.en_us_name);

			if item_class_array_idx == 1750 {

				save(&mut item_class, &item_class_arrays);
				item_class = String::new();
				item_class.push_str(format!("{imports}public class Items{nr}{o}public static void register(IEventBus a){o}items.register(a);{c}public static final DeferredRegister<Item> items=DeferredRegister.create(ForgeRegistries.ITEMS,\"allitems\");public static final RegistryObject[] itemArray=new RegistryObject[]{o}", imports = IMPORTS, nr = item_class_arrays+1, o = "{", c = "}").as_str());
				item_class_array_idx = 0;
				item_class_arrays+=1;

			}
			item_class_array_idx+=1;
//
			item_class.push_str(format!("items.register(\"{}\",()->new Item(new Item.Properties().tab(MiscElements.CTAB){})),", item_name, match item_type.special {
				ItemTypeSpecial::No => String::from_str("").unwrap(),
				ItemTypeSpecial::Fruit => format!(".food(new FoodProperties.Builder().alwaysEat().saturationMod({}f).nutrition({}).build())", material.tier as f32 / 1000.0 * 10.00, (material.tier as f32 / 1000.0 * 10.0 + 0.5) as u8),
			}).as_str());
			en_us_lang_file.push_str(format!(",\"item.allitems.{}\":\"{}\"", item_name, item_en_us_name).as_str());

		}

	}

	save(&mut item_class, &item_class_arrays);

}

fn save(item_class: &mut String, nr: &i32) {

	item_class.push_str("};}\n");
	File::create(format!("../src/main/java/com/tandex/allitems/scriptGen/Items{}.java", nr).as_str()).unwrap().write(item_class.as_bytes()).unwrap();

}
