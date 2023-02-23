package block;

import world.Chunk;

public final class BlockState {
    private final Block block;

    public final byte[] properties;
    private final boolean isFullCube;
    private final boolean[] cullsFace;
    public final BlockModel model;

    public BlockState(Block block, byte... properties) {
        if (properties.length != block.attributes.length) throw new RuntimeException();
        this.block = block;
        this.properties = properties;
        this.model = block.getModel(this);
        this.cullsFace = new boolean[6];
        this.isFullCube = block.isFullCube(this);

        if (this.model == null) return;
        short[][][] faces = this.model.faces;
        for (int i = 0; i < faces.length; i++) {
            short[][] dir = faces[i];
            for (short[] face : dir) {
                if (face[0] == 0 && face[1] == 0 && face[2] == 0 && face[3] == 16 && face[4] == 16) {
                    this.cullsFace[i % 2 == 0 ? i + 1 : i - 1] = true;
                    break;
                }
            }
        }
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

    public boolean hasModel() {
        return this.model != null;
    }

    public boolean cullsFace(Chunk.Face face) {
        return this.cullsFace[face.ordinal()];
    }
}
