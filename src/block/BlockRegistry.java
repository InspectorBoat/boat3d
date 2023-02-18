package block;

import java.util.ArrayList;

public class BlockRegistry {
    public final static BlockState[] registry;
    public static int totalStates = 0;

    private static void register(ArrayList<Block> blocks, Block block) {
        blocks.add(block);
        totalStates += block.getTotalStates();
    }

    static {
        final ArrayList<Block> blocks = new ArrayList<>();
        register(blocks, new AirBlock());
        register(blocks, new WoolBlock());
        register(blocks, new GrassBlock());

        registry = new BlockState[totalStates];
        int index = 0;
        for (Block block : blocks) {
            index = block.genStates(registry, index);
        }

        for (BlockState state : registry) {
//            System.out.println(state);
        }
    }
}
