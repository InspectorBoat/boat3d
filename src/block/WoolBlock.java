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
        return new BlockModel(new short[][][] {
                {{0, 0, 0, 16, 16, (short) (blockState.properties[0] + 3)}},
                {{0, 0, 0, 16, 16, (short) (blockState.properties[0] + 3)}},
                {{0, 0, 0, 16, 16, (short) (blockState.properties[0] + 3)}},
                {{0, 0, 0, 16, 16, (short) (blockState.properties[0] + 3)}},
                {{0, 0, 0, 16, 16, (short) (blockState.properties[0] + 3)}},
                {{0, 0, 0, 16, 16, (short) (blockState.properties[0] + 3)}},
        });
    }

}
