package boat.block;

import boat.world.Chunk;

public final class BlockState {
    private final Block block;

    public final byte[] properties;
    private final boolean isFullCube;
    private final short[][] cullProfile;
    public final BlockModel model;

    public BlockState(Block block, byte... properties) {
        if (properties.length != block.attributes.length) throw new RuntimeException();
        this.block = block;
        this.properties = properties;
        this.model = block.getModel(this);
        this.cullProfile = new short[6][4];
        this.isFullCube = block.isFullCube(this);

        for (int f = 0; f < 6; f ++) {
            for (int c = 0; c < 4; c ++) {
                this.cullProfile[f][c] = -1;
            }
        }

        if (this.model == null) return;
        int[][] faces = this.model.faces;
        for (int[] face : faces) {
            int normal = face[0] & 0x000000ff;
            int x = (face[0] & 0xff000000) >> 24;
            int y = (face[0] & 0x00ff0000) >> 16;
            int z = (face[0] & 0x0000ff00) >> 8;
            int w = (face[1] & 0xff000000) >> 24;
            int h = (face[1] & 0x00ff0000) >> 16;
            switch (normal) {
                case 0, 1 -> {
                    if (normal == 0 && z != 0) continue;
                    if (normal == 1 && z != 15) continue;
                    this.cullProfile[normal][0] = (short) (x);
                    this.cullProfile[normal][1] = (short) (y);
                    this.cullProfile[normal][2] = (short) (x + w + 1);
                    this.cullProfile[normal][3] = (short) (x + h + 1);
                }
                case 2, 3 -> {
                    if (normal == 2 && x != 0) continue;
                    if (normal == 3 && x != 15) continue;
                    this.cullProfile[normal][0] = (short) (z);
                    this.cullProfile[normal][1] = (short) (y);
                    this.cullProfile[normal][2] = (short) (z + w + 1);
                    this.cullProfile[normal][3] = (short) (y + h + 1);
                }
                case 4, 5 -> {
                    if (normal == 4 && y != 0) continue;
                    if (normal == 5 && y != 15) continue;
                    this.cullProfile[normal][0] = (short) (x);
                    this.cullProfile[normal][1] = (short) (z);
                    this.cullProfile[normal][2] = (short) (x + w + 1);
                    this.cullProfile[normal][3] = (short) (z + h + 1);
                }
                case default -> {
                    throw new RuntimeException("Unexpected normal");
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

    public short[] cullProfile(Chunk.Face face, boolean opposing) {
        if (opposing) return this.cullProfile[face.ordinal() + ((face.ordinal() % 2) * -2) + 1];
        return this.cullProfile[face.ordinal()];
    }
}
