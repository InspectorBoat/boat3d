package block;

import java.util.ArrayList;

public class BlockRegistry {
    public final static BlockState[] registry;
    public static int totalStates = 0;
    private static void register(ArrayList<Block> queue, Block block) {
        queue.add(block);
        totalStates += block.getTotalStates();
    }

    static {
        final ArrayList<Block> blocks = new ArrayList<>();
        register(blocks, new Block("air"));
        register(blocks, new Block("grass_block"));
        register(blocks, new Block("stone"));
        register(blocks, new Block("wool",
                                   new BlockAttribute("color", BlockProperty.COLOR)));
        register(blocks, new Block("redstone_dust",
                                   new BlockAttribute("power", BlockProperty.POWER),
                                   new BlockAttribute("north", BlockProperty.WIRE_CONNECTION),
                                   new BlockAttribute("east", BlockProperty.WIRE_CONNECTION),
                                   new BlockAttribute("south", BlockProperty.WIRE_CONNECTION),
                                   new BlockAttribute("west", BlockProperty.WIRE_CONNECTION),
                                   new BlockAttribute("channel1", new BlockProperty(15))));
        register(blocks, new Block("bed",
                                   new BlockAttribute("half", new BlockProperty.NamedBlockProperty("head", "foot")),
                                   new BlockAttribute("color", BlockProperty.COLOR)));
        register(blocks, new Block("concrete",
                                   new BlockAttribute("color", BlockProperty.COLOR)));
        register(blocks, new Block("concrete_powder",
                                   new BlockAttribute("color", BlockProperty.COLOR)));
        register(blocks, new Block("nether_portal",
                                   new BlockAttribute("face", BlockProperty.HORIZONTAL_AXIS)));
        register(blocks, new Block("log",
                                   new BlockAttribute("axis", BlockProperty.AXIS),
                                   new BlockAttribute("stripped", BlockProperty.BOOLEAN),
                                   new BlockAttribute("bark", BlockProperty.BOOLEAN),
                                   new BlockAttribute("species", BlockProperty.WOOD_SPECIES)));
        register(blocks, new Block("plank",
                                   new BlockAttribute("species", new BlockProperty.NamedBlockProperty())));
        register(blocks, new Block("stone_slab",
                                   new BlockAttribute("half", new BlockProperty.NamedBlockProperty("top", "bottom", "double"))));
        registry = new BlockState[totalStates];
        int index = 0;
        for (Block block : blocks) {
            index = block.genTotalStates(index);
        }
    }
}
