package block;

public abstract class Block {
    public final BlockAttribute[] attributes;
    public final String name;
    public final int totalStates;
    private boolean initialized;

    protected Block(String name, BlockAttribute... attributes) {
        //noinspection ConstantConditions
        if (this.initialized) throw new RuntimeException("Cannot reconstruct class");
        this.initialized = true;

        this.name = name;
        this.attributes = attributes;
        int states = 1;
        for (BlockAttribute attribute : this.attributes) {
            states *= attribute.property().count + 1;
        }
        this.totalStates = states;
    }

    protected int getTotalStates() {
        return this.totalStates;
    }

    protected int genStates(BlockState[] registry, int index) {
        byte[][] states = new byte[this.totalStates][this.attributes.length];

        int blockSize = 1;
        for (int prop = 0; prop < this.attributes.length; prop++) {
            int size = this.attributes[prop].property().count + 1;
            for (int i = 0; i < this.totalStates; i++) {
                states[i][prop] = (byte) ((i / blockSize) % size);
            }
            blockSize *= size;
        }
        for (int i = 0; i < this.totalStates; index++) {
            registry[index] = new BlockState(this, states[i++]);
        }
        return index;
    }

    protected abstract boolean isFullCube(BlockState blockState);

    public abstract BlockModel getModel(BlockState blockState);
}
