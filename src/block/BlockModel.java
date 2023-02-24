package block;

public class BlockModel {
    // depth, left, top, width, height, texture id
    public byte[][][] faces;
    public BlockModel(byte[][][] faces) {
        this.faces = faces;
    }
}