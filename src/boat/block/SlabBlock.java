package boat.block;

public class SlabBlock extends Block {
    protected SlabBlock() {
        super("slab");
    }

    @Override
    protected boolean isFullCube(BlockState blockState) {
        return false;
    }

    @Override
    public BlockModel getModel(BlockState blockState) {
        return new BlockModel(new int[][] {
                {0x00000000, 0x0f080f00},
                {0x00000f01, 0x0f080f00},
                {0x00000002, 0x0f080f00},
                {0x0f000003, 0x0f080f00},
                {0x00000004, 0x0f0f0f00},
                {0x00080005, 0x0f0f0f00},
        });
    }
}
