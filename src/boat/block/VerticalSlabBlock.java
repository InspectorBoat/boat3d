package boat.block;

public class VerticalSlabBlock extends Block {
    protected VerticalSlabBlock() {
        super("vertical_slab", new BlockAttribute("left", BlockProperty.BOOLEAN));
    }

    @Override
    protected boolean isFullCube(BlockState blockState) {
        return false;
    }

    @Override
    public BlockModel getModel(BlockState blockState) {
        if (blockState.properties[0] == 0) return new BlockModel(new int[][] {
                {0x00000000, 0x070f0f01},
                {0x00000001, 0x0f070f01},
                {0x00000002, 0x0f070f01},
                {0x00000003, 0x0f070f01},
                {0x00000004, 0x0f0f0f01},
                {0x00000005, 0x0f0f0f01},
        });
        return new BlockModel(new int[][] {
                {0x00080000, 0x0f070f02},
                {0x00080f01, 0x0f070f02},
                {0x00080002, 0x0f070f02},
                {0x0f080003, 0x0f070f02},
                {0x00080004, 0x0f0f0f02},
                {0x000f0005, 0x0f0f0f02},
        });
    }
}
