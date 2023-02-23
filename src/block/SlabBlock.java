package block;

public class SlabBlock extends Block {
    protected SlabBlock(String name, BlockAttribute... attributes) {
        super(name, attributes);
    }

    @Override
    protected boolean isFullCube(BlockState blockState) {
        return false;
    }

    @Override
    public BlockModel getModel(BlockState blockState) {
        return new BlockModel(new short[][][] {
                {{0, 0, 0, 16, 8, 0}},
                {{0, 0, 0, 16, 8, 0}},
                {{0, 0, 0, 16, 8, 0}},
                {{0, 0, 0, 16, 8, 0}},
                {{0, 0, 0, 16, 16, 0}},
                {{8, 0, 0, 16, 16, 0}},
        });
    }
}
