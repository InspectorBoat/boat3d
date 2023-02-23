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

    public static short[] genVertices(World world, Chunk chunk, BlockState block, Face face, short x, short y, short z) {
        int offset = dir(face, x, y, z);
        boolean faceCulled = false;
        if (offset >= 0 && offset <= 4095) {
            if (chunk.blocks[offset].cullsFace(face)) {
                faceCulled = true;
            }
        } else {
            int chunkOffset = dir(face, (short) chunk.chunkPos.x, (short) chunk.chunkPos.y, (short) chunk.chunkPos.z);
            if (chunkOffset >= 0 && chunkOffset <= 4095 && world.chunks[chunkOffset] != null && world.chunks[chunkOffset].blocks != null) {
                if (world.chunks[chunkOffset].blocks[offset - 8192] != null && world.chunks[chunkOffset].blocks[offset - 8192].cullsFace(face)) {
                    faceCulled = true;
                }
            }
        }
        switch (face) {
            case NORTH -> {
                for (short[] faceInfo : block.model.faces[face.ordinal()]) {
                    short depth = faceInfo[0];
                    if (faceCulled && depth == 0) return null;

                    short x1 = faceInfo[1];
                    short y1 = faceInfo[2];
                    short x2 = faceInfo[3];
                    short y2 = faceInfo[4];
                    short blockX = (short) (x * 16);
                    short blockY = (short) (y * 16);
                    short blockZ = (short) (z * 16);
                    short texture = faceInfo[5];
                    return new short[] {
                            (short) (blockX + x1), (short) (blockY + y2), (short) (blockZ + depth), 0b00,
                            (short) (blockX + x1), (short) (blockY + y1), (short) (blockZ + depth), 0b01,
                            (short) (blockX + x2), (short) (blockY + y2), (short) (blockZ + depth), 0b10,
                            (short) (blockX + x2), (short) (blockY + y1), (short) (blockZ + depth), 0b11,
                            texture
                    };
                }
                break;
            }
            case SOUTH -> {
                for (short[] faceInfo : block.model.faces[face.ordinal()]) {
                    short depth = faceInfo[0];
                    if (faceCulled && depth == 0) return null;

                    short x1 = faceInfo[1];
                    short y1 = faceInfo[2];
                    short x2 = faceInfo[3];
                    short y2 = faceInfo[4];
                    short texture = faceInfo[5];
                    short blockX = (short) (x * 16);
                    short blockY = (short) (y * 16);
                    short blockZ = (short) (z * 16 + 16);
                    return new short[] {
                            (short) (blockX + x2), (short) (blockY + y2), (short) (blockZ - depth), 0b00,
                            (short) (blockX + x2), (short) (blockY + y1), (short) (blockZ - depth), 0b01,
                            (short) (blockX + x1), (short) (blockY + y2), (short) (blockZ - depth), 0b10,
                            (short) (blockX + x1), (short) (blockY + y1), (short) (blockZ - depth), 0b11,
                            texture
                    };
                }
            }
            case WEST -> {
                for (short[] faceInfo : block.model.faces[face.ordinal()]) {
                    short depth = faceInfo[0];
                    if (faceCulled && depth == 0) return null;

                    short z1 = faceInfo[1];
                    short y1 = faceInfo[2];
                    short z2 = faceInfo[3];
                    short y2 = faceInfo[4];
                    short texture = faceInfo[5];
                    short blockX = (short) (x * 16);
                    short blockY = (short) (y * 16);
                    short blockZ = (short) (z * 16);
                    return new short[] {
                            (short) (blockX + depth), (short) (blockY + y2), (short) (blockZ + z2), 0b00,
                            (short) (blockX + depth), (short) (blockY + y1), (short) (blockZ + z2), 0b01,
                            (short) (blockX + depth), (short) (blockY + y2), (short) (blockZ + z1), 0b10,
                            (short) (blockX + depth), (short) (blockY + y1), (short) (blockZ + z1), 0b11,
                            texture
                    };
                }
            }
            case EAST -> {
                for (short[] faceInfo : block.model.faces[face.ordinal()]) {
                    short depth = faceInfo[0];
                    if (faceCulled && depth == 0) return null;

                    short z1 = faceInfo[1];
                    short y1 = faceInfo[2];
                    short z2 = faceInfo[3];
                    short y2 = faceInfo[4];
                    short texture = faceInfo[5];
                    short blockX = (short) (x * 16 + 16);
                    short blockY = (short) (y * 16);
                    short blockZ = (short) (z * 16);
                    return new short[] {
                            (short) (blockX - depth), (short) (blockY + y2), (short) (blockZ + z1), 0b00,
                            (short) (blockX - depth), (short) (blockY + y1), (short) (blockZ + z1), 0b01,
                            (short) (blockX - depth), (short) (blockY + y2), (short) (blockZ + z2), 0b10,
                            (short) (blockX - depth), (short) (blockY + y1), (short) (blockZ + z2), 0b11,
                            texture
                    };
                }
            }
            case DOWN -> {
                for (short[] faceInfo : block.model.faces[face.ordinal()]) {
                    short depth = faceInfo[0];
                    if (faceCulled && depth == 0) return null;

                    short x1 = faceInfo[1];
                    short z1 = faceInfo[2];
                    short x2 = faceInfo[3];
                    short z2 = faceInfo[4];
                    short texture = faceInfo[5];
                    short blockX = (short) (x * 16);
                    short blockY = (short) (y * 16);
                    short blockZ = (short) (z * 16);
                    return new short[] {
                            (short) (blockX + x1), (short) (blockY + depth), (short) (blockZ + z1), 0b00,
                            (short) (blockX + x1), (short) (blockY + depth), (short) (blockZ + z2), 0b01,
                            (short) (blockX + x2), (short) (blockY + depth), (short) (blockZ + z1), 0b10,
                            (short) (blockX + x2), (short) (blockY + depth), (short) (blockZ + z2), 0b11,
                            texture
                    };
                }
            }
            case UP -> {
                for (short[] faceInfo : block.model.faces[face.ordinal()]) {
                    short depth = faceInfo[0];
                    if (faceCulled && depth == 0) return null;

                    short x1 = faceInfo[1];
                    short z1 = faceInfo[2];
                    short x2 = faceInfo[3];
                    short z2 = faceInfo[4];
                    short texture = faceInfo[5];
                    short blockX = (short) (x * 16);
                    short blockY = (short) (y * 16 + 16);
                    short blockZ = (short) (z * 16);
                    return new short[]{
                            (short) (blockX + x1), (short) (blockY - depth), (short) (blockZ + z2), 0b00,
                            (short) (blockX + x1), (short) (blockY - depth), (short) (blockZ + z1), 0b01,
                            (short) (blockX + x2), (short) (blockY - depth), (short) (blockZ + z2), 0b10,
                            (short) (blockX + x2), (short) (blockY - depth), (short) (blockZ + z1), 0b11,
                            texture
                    };
                }
            }
        }

        return null;
    }

    public static int dir(Face face, short x, short y, short z) {
        switch (face) {
            case NORTH -> {
                return z <= 0  ?
                       ((x << 8) | (y << 4) | ((15) << 0)) + 8192:
                       (x << 8) | (y << 4) | ((z - 1) << 0);
            }
            case SOUTH -> {
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
