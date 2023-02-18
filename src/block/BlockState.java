package block;

import world.Chunk;

public final class BlockState {
    private final Block block;

    public final byte[] properties;
    private final boolean isFullCube;
    private final short[] textureIds;

    public BlockState(Block block, byte... properties) {
        if (properties.length != block.attributes.length) throw new RuntimeException();
        this.block = block;
        this.properties = properties;
        this.isFullCube = block.isFullCube(this);
        this.textureIds = new short[]{
                this.block.getTextureId(this, Chunk.Face.NORTH),
                this.block.getTextureId(this, Chunk.Face.SOUTH),
                this.block.getTextureId(this, Chunk.Face.EAST),
                this.block.getTextureId(this, Chunk.Face.WEST),
                this.block.getTextureId(this, Chunk.Face.DOWN),
                this.block.getTextureId(this, Chunk.Face.UP),
        };
    }

    public String toString() {
        StringBuilder str = new StringBuilder(this.block.name);
        str.append(":[");
        for (int j = 0; j < this.properties.length; j++) {
            int i = this.properties[j];
            str.append(this.block.attributes[j].name());
            str.append('=');
            if (this.block.attributes[j].property() instanceof BlockProperty.NamedBlockProperty) {
                str.append('\'');
                str.append(((BlockProperty.NamedBlockProperty) this.block.attributes[j].property()).getName(i));
                str.append('\'');
            }
            else str.append(i);
            if (j < this.properties.length - 1) str.append(',');
        }
        str.append("]");
        return str.toString();
    }

    public boolean isFullCube() {
        return this.isFullCube;
    }

    public short getTexture(Chunk.Face face) {
        return this.textureIds[face.ordinal()];
    }
}
