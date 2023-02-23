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
        return new BlockModel(new short[][][] {
                {{0, 0, 0, 16, 16, 0}},
                {{0, 0, 0, 16, 16, 0}},
                {{0, 0, 0, 16, 16, 0}},
                {{0, 0, 0, 16, 16, 0}},
                {{0, 0, 0, 16, 16, 0}},
                {{0, 0, 0, 16, 16, 0}},
        });
    }
}
