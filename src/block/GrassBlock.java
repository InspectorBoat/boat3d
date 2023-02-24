package block;

public class GrassBlock extends Block {
    public GrassBlock() {
        super("grass", new BlockAttribute("snowy", BlockProperty.BOOLEAN));
//        super("grass");
    }

    @Override
    protected boolean isFullCube(BlockState blockState) {
        return true;
    }

    @Override
    public BlockModel getModel(BlockState blockState) {
        return new BlockModel(new int[][] {
                {0x00000000, 0x0f0f0f00},
                {0x00000f01, 0x0f0f0f00},
                {0x00000002, 0x0f0f0f00},
                {0x0f000003, 0x0f0f0f00},
                {0x00000004, 0x0f0f0f00},
                {0x000f0005, 0x0f0f0f00},
        });
    }
}
