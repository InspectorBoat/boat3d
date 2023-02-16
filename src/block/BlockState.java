package block;

public final class BlockState {
    private final Block block;
    public final byte[] properties;
    public final boolean isFullCube;

    public BlockState(Block block, byte... properties)
    {
        if (properties.length != block.attributes.length) throw new RuntimeException();
        this.block = block;
        this.properties = properties;
        isFullCube = Math.random() > 0.2;
    }

    public String toString()
    {
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
}
