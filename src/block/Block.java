package block;

public class Block {
    public final BlockAttribute[] attributes;
    public final String name;
    public final int totalStates;

    Block(String name, BlockAttribute...attributes) {
        this.name = name;
        this.attributes = attributes;
        int states = 1;
        for (BlockAttribute attribute : this.attributes) {
            states *= attribute.property().max + 1;
        }
        this.totalStates = states;
    }
    public int getTotalStates()
    {
        return this.totalStates;
    }

    protected int genTotalStates(int index) {
        byte[][] states = new byte[this.totalStates][this.attributes.length];

        int blockSize = 1;
        for (int prop = 0; prop < this.attributes.length; prop ++) {
            int size = this.attributes[prop].property().max + 1;
            for (int i = 0; i < this.totalStates; i ++) {
                states[i][prop] = (byte) ((i / blockSize) % size);
            }
            blockSize *= size;
        }
        for (int i = 0; i < this.totalStates; i ++, index ++) {
            BlockRegistry.registry[index] = new BlockState(this, states[i]);
        }
        return index;
    }
}
