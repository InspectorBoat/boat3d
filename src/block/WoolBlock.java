package block;

import world.Chunk;

public class WoolBlock extends Block {
    public WoolBlock() {
        super("wool", new BlockAttribute("color", BlockProperty.COLOR));
    }

    @Override
    public boolean isFullCube(BlockState blockState) {
        return true;
    }

    @Override
    public short getTextureId(BlockState block, Chunk.Face face) {
        return (short) (3 + block.properties[0] * 5000);
    }
}
