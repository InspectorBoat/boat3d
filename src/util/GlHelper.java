package util;

import block.BlockState;
import world.Chunk;
import world.Chunk.Face;
import world.World;

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

    public static byte[] genVertices(World world, Chunk chunk, BlockState block, Face face, byte x, byte y, byte z) {
        int offset = dir(face, x, y, z);
//        boolean faceCulled = false;
//        if (offset >= 0 && offset <= 4095) {
//            if (chunk.blocks[offset].cullsFace(face)) {
//                faceCulled = true;
//            }
//        } else {
//            int chunkOffset = dir(face, (byte) chunk.chunkPos.x, (byte) chunk.chunkPos.y, (byte) chunk.chunkPos.z);
//            if (chunkOffset >= 0 && chunkOffset <= 4095 && world.chunks[chunkOffset] != null && world.chunks[chunkOffset].blocks != null) {
//                if (world.chunks[chunkOffset].blocks[offset - 8192] != null && world.chunks[chunkOffset].blocks[offset - 8192].cullsFace(face)) {
//                    faceCulled = true;
//                }
//            }
//        }

        return null;
    }

    public static int dir(Face face, byte x, byte y, byte z) {
        switch (face) {
            case SOUTH -> {
                return z <= 0  ?
                       ((x << 8) | (y << 4) | ((15) << 0)) + 8192:
                       (x << 8) | (y << 4) | ((z - 1) << 0);
            }
            case NORTH -> {
                return z >= 15 ?
                       ((x << 8) | (y << 4) | ((0) << 0)) + 8192:
                       (x << 8) | (y << 4) | ((z + 1) << 0);
            }
            case WEST -> {
                return x <= 0  ?
                       (((15) << 8) | (y << 4) | (z << 0)) + 8192:
                       ((x - 1) << 8) | (y << 4) | (z << 0);
            }
            case EAST -> {
                return x >= 15 ?
                       (((0) << 8) | (y << 4) | (z << 0)) + 8192:
                       ((x + 1) << 8) | (y << 4) | (z << 0);
            }
            case DOWN -> {
                return y <= 0  ?
                       ((x << 8) | ((15) << 4) | (z << 0)) + 8192:
                       (x << 8) | ((y - 1) << 4) | (z << 0);
            }
            case UP -> {
                return y >= 15 ?
                       ((x << 8) | ((0) << 4) | (z << 0)) + 8192:
                       (x << 8) | ((y + 1) << 4) | (z << 0);
            }
        }
        throw new RuntimeException();
    }
}
