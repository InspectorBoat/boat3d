package block;

public class BlockModel {
    // depth, left, top, width, height, texture id
    public int[][] faces;
    public BlockModel(int[][] faces) {
        this.faces = faces;
    }
}