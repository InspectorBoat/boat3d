package block;

import world.Chunk;

public class AirBlock extends Block {
    protected AirBlock() {
        super("air");
    }

    @Override
    public boolean isFullCube(BlockState block) {
        return false;
    }

    @Override
    public short getTextureId(BlockState block, Chunk.Face face) {
        return 0;
    }
}
