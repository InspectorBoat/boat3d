package block;

import world.Chunk;

public class GrassBlock extends Block {
    public GrassBlock() {
        super("grass", new BlockAttribute("snowy", BlockProperty.BOOLEAN));
    }

    @Override
    protected boolean isFullCube(BlockState blockState) {
        return true;
    }

    @Override
    public short getTextureId(BlockState block, Chunk.Face face) {
        if (face == Chunk.Face.UP) {
            if (block.properties[0] == 1) return 1;
            return 0;
        }
        return 2;
    }
}
