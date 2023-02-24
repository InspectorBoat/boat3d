package block;

public class WoolBlock extends Block {
    public WoolBlock() {
        super("wool", new BlockAttribute("color", BlockProperty.COLOR));
    }

    @Override
    public boolean isFullCube(BlockState blockState) {
        return true;
    }

    @Override
    public BlockModel getModel(BlockState blockState) {
        return new BlockModel(new int[][] {
                {0x00000000, 0x0f0f0f00},
                {0x00000001, 0x0f0f0f00},
                {0x00000002, 0x0f0f0f00},
                {0x00000003, 0x0f0f0f00},
                {0x00000004, 0x0f0f0f00},
                {0x00000005, 0x0f0f0f00},
        });
    }

}
