package boat.block;

public class BlockModel {
    public int[][] faces;
    public BlockModel(int[][] faces) {
        this.faces = faces;
    }

    public static int faceTexture(int width, int height, int texture) {
        return width << 24 | height << 16 | texture;
    }

    public static int width(int[] face) {
        return (face[1] & 0xff000000) >> 24;
    }
    public static int height(int[] face) {
        return (face[1] & 0x00ff0000) >> 16;
    }
    public static int texture(int[] face) {
        return (face[1] & 0x0000ffff);
    }
    public static int x(int[] face) {
        return (face[0] & 0xff000000) >> 24;
    }
    public static int y(int[] face) {
        return (face[0] & 0x00ff0000) >> 16;
    }
    public static int z(int[] face) {
        return (face[0] & 0x0000ff00) >> 8;
    }
    public static int normal(int[] face) {
        return (face[0] & 0x000000ff);
    }

    public static int width(int face) {
        return (face & 0xff000000) >> 24;
    }
    public static int height(int face) {
        return (face & 0x00ff0000) >> 16;
    }
    public static int texture(int face) {
        return (face & 0x0000ffff);
    }
    public static int x(int face) {
        return (face & 0xff000000) >> 24;
    }
    public static int y(int face) {
        return (face & 0x00ff0000) >> 16;
    }
    public static int z(int face) {
        return (face & 0x0000ff00) >> 8;
    }
    public static int normal(int face) {
        return (face & 0x000000ff);
    }


    public static int rawWidth(int[] face) {
        return (face[1] & 0xff000000);
    }
    public static int rawHeight(int[] face) {
        return (face[1] & 0x00ff0000);
    }
    public static int rawTexture(int[] face) {
        return (face[1] & 0x0000ffff);
    }
    public static int rawX(int[] face) {
        return (face[0] & 0xff000000);
    }
    public static int rawY(int[] face) {
        return (face[0] & 0x00ff0000);
    }
    public static int rawZ(int[] face) {
        return (face[0] & 0x0000ff00);
    }
    public static int rawNormal(int[] face) {
        return (face[0] & 0x000000ff);
    }

}