package util;

import block.BlockState;
import world.Chunk;
import world.Chunk.Face;

import static org.lwjgl.opengl.GL46C.*;

public class GlHelper {
    public static int createShader(int type, String source) {
        int shader = glCreateShader(type);
        glShaderSource(shader, source);
        glCompileShader(shader);
        int[] status = new int[1];
        glGetShaderiv(shader, GL_COMPILE_STATUS, status);
        if (status[0] == GL_TRUE) return shader;
        throw new RuntimeException("Failed to compile " + (type == GL_FRAGMENT_SHADER ? "fragment" : "vertex") + " shader");
    }

    public static int createProgram(int vertShader, int fragShader) {
        int program = glCreateProgram();
        glAttachShader(program, vertShader);
        glAttachShader(program, fragShader);
        glLinkProgram(program);
        int[] status = new int[1];
        glGetProgramiv(program, GL_LINK_STATUS, status);
        if (status[0] == GL_TRUE) return program;
        throw new RuntimeException("Failed to link program");
    }

    public static short[] genVertices(Chunk chunk, BlockState block, Face face, short x, short y, short z) {
        int offset = dir(face, x, y, z);
        if (offset >= 0 && offset <= 4095) {
            if (chunk.blocks[offset].isFullCube()) return null;
        }
        switch (face) {
            case NORTH -> {
                return new short[]{
                        x, y, z,
                        (short) (x + 1), y, z,
                        (short) (x + 1), (short) (y + 1), z,
                        x, (short) (y + 1), z,
//                        (short) (x + 1), y, z,
//                        x, (short) (y + 1), z,
                        block.getTexture(face)
                };
            }
            case SOUTH -> {
                return new short[]{
                        x, y, (short) (z + 1),
                        x, (short) (y + 1), (short) (z + 1),
                        (short) (x + 1), (short) (y + 1), (short) (z + 1),
                        (short) (x + 1), y, (short) (z + 1),
//                        (short) (x + 1), y, (short) (z + 1),
//                        x, (short) (y + 1), (short) (z + 1),
                        block.getTexture(face)
                };
            }
            case WEST -> {
                return new short[]{
                        x, y, z,
                        x, (short) (y + 1), z,
                        x, (short) (y + 1), (short) (z + 1),
                        x, y, (short) (z + 1),
//                        x, y, (short) (z + 1),
//                        x, (short) (y + 1), z,
                        block.getTexture(face)
                };
            }
            case EAST -> {
                return new short[]{
                        (short) (x + 1), y, z,
                        (short) (x + 1), y, (short) (z + 1),
                        (short) (x + 1), (short) (y + 1), (short) (z + 1),
                        (short) (x + 1), (short) (y + 1), z,
//                        (short) (x + 1), y, (short) (z + 1),
//                        (short) (x + 1), (short) (y + 1), z,
                        block.getTexture(face)
                };
            }
            case DOWN -> {
                return new short[]{
                        x, y, z,
                        x, y, (short) (z + 1),
                        (short) (x + 1), y, (short) (z + 1),
                        (short) (x + 1), y, z,
//                        (short) (x + 1), y, z,
//                        x, y, (short) (z + 1),
                        block.getTexture(face)
                };
            }
            case UP -> {
                return new short[]{
                        x, (short) (y + 1), z,
                        (short) (x + 1), (short) (y + 1), z,
                        (short) (x + 1), (short) (y + 1), (short) (z + 1),
                        x, (short) (y + 1), (short) (z + 1),
//                        (short) (x + 1), (short) (y + 1), z,
//                        x, (short) (y + 1), (short) (z + 1),
                        block.getTexture(face)
                };
            }
        }
        return null;
    }

    public static int dir(Face face, short x, short y, short z) {
        switch (face) {
            case NORTH -> {
                return z <= 0 ? -1 : (x << 8) | (y << 4) | (z - 1);
            }
            case SOUTH -> {
                return z >= 15 ? -1 : (x << 8) | (y << 4) | (z + 1);
            }
            case WEST -> {
                return x <= 0 ? -1 : ((x - 1) << 8) | (y << 4) | (z);
            }
            case EAST -> {
                return x >= 15 ? -1 : ((x + 1) << 8) | (y << 4) | (z);
            }
            case DOWN -> {
                return y <= 0 ? -1 : (x << 8) | ((y - 1) << 4) | (z);
            }
            case UP -> {
                return y >= 15 ? -1 : (x << 8) | ((y + 1) << 4) | (z);
            }
        }
        throw new RuntimeException();
    }
}
