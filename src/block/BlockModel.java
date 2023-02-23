package block;

public class BlockModel {
    // depth, left, top, width, height, texture id
    public short[][][] faces;
    public BlockModel(short[][][] faces) {
        this.faces = faces;
    }
}