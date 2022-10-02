package com.tandex.allitems;

import net.minecraft.world.item.CreativeModeTab;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.item.Items;

public class MiscElements {
	
	public static final CreativeModeTab CTAB = new CreativeModeTab("allitems_creative_tab") {
		public ItemStack makeIcon() {
			return new ItemStack(Items.AIR);
		}
	};
	
}
