package com.tandex.allitems;

import com.mojang.logging.LogUtils;
import com.tandex.allitems.scriptGen.*;
//import com.tandex.allitems.test.Elements;
//import net.minecraft.world.level.block.Blocks;
import net.minecraftforge.common.MinecraftForge;
import net.minecraftforge.eventbus.api.IEventBus;
import net.minecraftforge.fml.common.Mod;
import net.minecraftforge.fml.event.lifecycle.FMLCommonSetupEvent;
import net.minecraftforge.fml.javafmlmod.FMLJavaModLoadingContext;
import org.slf4j.Logger;
//import java.awt.Color;

/// The value here should match an entry in the META-INF/mods.toml file
@Mod("allitems")
public class Allitems {
	// Directly reference a slf4j logger
	private static final Logger LOGGER = LogUtils.getLogger();
	//public static final Color ideaChoseColor = new Color(0x51B318);
	
	public long registerStartTime;
	public long registerEndTime;
	
	public Allitems() {
		registerStartTime = System.currentTimeMillis();
		// Register the setup method for modloading
		IEventBus eventBus = FMLJavaModLoadingContext.get().getModEventBus();
		//Elements.register(eventBus);
		///*
		Items0.register(eventBus); // make all Items classes that currently exists (or all you want)
		Items1.register(eventBus);
		Items2.register(eventBus);
		Items3.register(eventBus);
		Items4.register(eventBus);
		Items5.register(eventBus);
		Items6.register(eventBus);
		//*/
		eventBus.addListener(this::setup);
		
		// Register ourselves for server and other game events we are interested in
		MinecraftForge.EVENT_BUS.register(this);
		registerEndTime = System.currentTimeMillis();
	}
	
	private void setup(final FMLCommonSetupEvent event) {
		
		LOGGER.info("allitems loading, I'm interested do you will even read this");
		LOGGER.info("btw object created or just all things were registered in " + (registerEndTime - registerStartTime) + "ms");
		
	}
	
}
