package com.tandex.allitems.test;

import com.tandex.allitems.MiscElements;
import net.minecraft.world.food.FoodProperties;
import net.minecraft.world.item.Item;
import net.minecraftforge.eventbus.api.IEventBus;
import net.minecraftforge.registries.DeferredRegister;
import net.minecraftforge.registries.ForgeRegistries;
import net.minecraftforge.registries.RegistryObject;

// in this file I was testing basics, nothing special

public class Elements {
	
	public static final DeferredRegister<Item> items = DeferredRegister.create(ForgeRegistries.ITEMS, "allitems");
	
	public static final RegistryObject<Item> item = items.register("huj", () -> new Item(new Item.Properties().tab(MiscElements.CTAB).food(new FoodProperties.Builder().alwaysEat().saturationMod(0.0f).nutrition(0).build())));
	
	public static void register(IEventBus eventBus) {
		items.register(eventBus);
	}
	
}
