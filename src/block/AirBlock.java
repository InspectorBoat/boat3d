package block;

public class AirBlock extends Block {
    protected AirBlock() {
        super("air");
    }

    @Override
    public boolean isFullCube(BlockState block) {
        return false;
    }

    @Override
    public BlockModel getModel(BlockState blockState) {
        return null;
    }
}
