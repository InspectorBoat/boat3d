package src.block;

import java.util.ArrayList;

public class Blocks {
    public final static BlockState[] states;
    public static int totalStates = 0;

    private static void register(ArrayList<Block> blocks, Block block) {
        blocks.add(block);
        totalStates += block.getTotalStates();
    }

    static {
        final ArrayList<Block> blocks = new ArrayList<>();
        register(blocks, new AirBlock());
        register(blocks, new GrassBlock());

        states = new BlockState[totalStates];
        int index = 0;
        for (Block block : blocks) {
            index = block.genStates(Blocks.states, index);
        }
    }
}
