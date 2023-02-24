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
        boolean faceCulled = false;
        if (offset >= 0 && offset <= 4095) {
            if (chunk.blocks[offset].cullsFace(face)) {
                faceCulled = true;
            }
        } else {
            int chunkOffset = dir(face, (byte) chunk.chunkPos.x, (byte) chunk.chunkPos.y, (byte) chunk.chunkPos.z);
            if (chunkOffset >= 0 && chunkOffset <= 4095 && world.chunks[chunkOffset] != null && world.chunks[chunkOffset].blocks != null) {
                if (world.chunks[chunkOffset].blocks[offset - 8192] != null && world.chunks[chunkOffset].blocks[offset - 8192].cullsFace(face)) {
                    faceCulled = true;
                }
            }
        }
        switch (face) {
            case NORTH -> {
                for (byte[] faceInfo : block.model.faces[face.ordinal()]) {
                    byte depth = faceInfo[0];
                    if (faceCulled && depth == 0) return null;

                    byte x1 = faceInfo[1];
                    byte y1 = faceInfo[2];
                    byte x2 = faceInfo[3];
                    byte y2 = faceInfo[4];
                    byte blockX = (byte) (x * 16);
                    byte blockY = (byte) (y * 16);
                    byte blockZ = (byte) (z * 16);
                    byte texture = faceInfo[5];
                    return new byte[] {
                            (byte) (blockX + x1), (byte) (blockY + y2), (byte) (blockZ + depth), 0b00,
                            (byte) (blockX + x1), (byte) (blockY + y1), (byte) (blockZ + depth), 0b01,
                            (byte) (blockX + x2), (byte) (blockY + y2), (byte) (blockZ + depth), 0b10,
                            (byte) (blockX + x2), (byte) (blockY + y1), (byte) (blockZ + depth), 0b11,
                            texture
                    };
                }
            }
            /*
            case SOUTH -> {
                for (byte[] faceInfo : block.model.faces[face.ordinal()]) {
                    byte depth = faceInfo[0];
                    if (faceCulled && depth == 0) return null;
                    if (z == 15) return null;
                    byte x1 = faceInfo[1];
                    byte y1 = faceInfo[2];
                    byte x2 = faceInfo[3];
                    byte y2 = faceInfo[4];
                    byte texture = faceInfo[5];
                    byte blockX = (byte) (x * 16);
                    byte blockY = (byte) (y * 16);
                    byte blockZ = (byte) (z * 16 + 16);
                    return new byte[] {
                            (byte) (blockX + x2), (byte) (blockY + y2), (byte) (blockZ - depth), 0b00,
                            (byte) (blockX + x2), (byte) (blockY + y1), (byte) (blockZ - depth), 0b01,
                            (byte) (blockX + x1), (byte) (blockY + y2), (byte) (blockZ - depth), 0b10,
                            (byte) (blockX + x1), (byte) (blockY + y1), (byte) (blockZ - depth), 0b11,
                            texture
                    };
                }
            }
            case WEST -> {
                for (byte[] faceInfo : block.model.faces[face.ordinal()]) {
                    byte depth = faceInfo[0];
                    if (faceCulled && depth == 0) return null;

                    byte z1 = faceInfo[1];
                    byte y1 = faceInfo[2];
                    byte z2 = faceInfo[3];
                    byte y2 = faceInfo[4];
                    byte texture = faceInfo[5];
                    byte blockX = (byte) (x * 16);
                    byte blockY = (byte) (y * 16);
                    byte blockZ = (byte) (z * 16);
                    return new byte[] {
                            (byte) (blockX + depth), (byte) (blockY + y2), (byte) (blockZ + z2), 0b00,
                            (byte) (blockX + depth), (byte) (blockY + y1), (byte) (blockZ + z2), 0b01,
                            (byte) (blockX + depth), (byte) (blockY + y2), (byte) (blockZ + z1), 0b10,
                            (byte) (blockX + depth), (byte) (blockY + y1), (byte) (blockZ + z1), 0b11,
                            texture
                    };
                }
            }
            case EAST -> {
                for (byte[] faceInfo : block.model.faces[face.ordinal()]) {
                    byte depth = faceInfo[0];
                    if (faceCulled && depth == 0) return null;
                    if (x == 15) return null;

                    byte z1 = faceInfo[1];
                    byte y1 = faceInfo[2];
                    byte z2 = faceInfo[3];
                    byte y2 = faceInfo[4];
                    byte texture = faceInfo[5];
                    byte blockX = (byte) (x * 16 + 16);
                    byte blockY = (byte) (y * 16);
                    byte blockZ = (byte) (z * 16);
                    return new byte[] {
                            (byte) (blockX - depth), (byte) (blockY + y2), (byte) (blockZ + z1), 0b00,
                            (byte) (blockX - depth), (byte) (blockY + y1), (byte) (blockZ + z1), 0b01,
                            (byte) (blockX - depth), (byte) (blockY + y2), (byte) (blockZ + z2), 0b10,
                            (byte) (blockX - depth), (byte) (blockY + y1), (byte) (blockZ + z2), 0b11,
                            texture
                    };
                }
            }
            case DOWN -> {
                for (byte[] faceInfo : block.model.faces[face.ordinal()]) {
                    byte depth = faceInfo[0];
                    if (faceCulled && depth == 0) return null;

                    byte x1 = faceInfo[1];
                    byte z1 = faceInfo[2];
                    byte x2 = faceInfo[3];
                    byte z2 = faceInfo[4];
                    byte texture = faceInfo[5];
                    byte blockX = (byte) (x * 16);
                    byte blockY = (byte) (y * 16);
                    byte blockZ = (byte) (z * 16);
                    return new byte[] {
                            (byte) (blockX + x1), (byte) (blockY + depth), (byte) (blockZ + z1), 0b00,
                            (byte) (blockX + x1), (byte) (blockY + depth), (byte) (blockZ + z2), 0b01,
                            (byte) (blockX + x2), (byte) (blockY + depth), (byte) (blockZ + z1), 0b10,
                            (byte) (blockX + x2), (byte) (blockY + depth), (byte) (blockZ + z2), 0b11,
                            texture
                    };
                }
            }
            case UP -> {
                for (byte[] faceInfo : block.model.faces[face.ordinal()]) {
                    byte depth = faceInfo[0];
                    if (faceCulled && depth == 0) return null;
                    if (y == 15) return null;

                    byte x1 = faceInfo[1];
                    byte z1 = faceInfo[2];
                    byte x2 = faceInfo[3];
                    byte z2 = faceInfo[4];
                    byte texture = faceInfo[5];
                    byte blockX = (byte) (x * 16);
                    byte blockY = (byte) (y * 16 + 16);
                    byte blockZ = (byte) (z * 16);
                    return new byte[]{
                            (byte) (blockX + x1), (byte) (blockY - depth), (byte) (blockZ + z2), 0b00,
                            (byte) (blockX + x1), (byte) (blockY - depth), (byte) (blockZ + z1), 0b01,
                            (byte) (blockX + x2), (byte) (blockY - depth), (byte) (blockZ + z2), 0b10,
                            (byte) (blockX + x2), (byte) (blockY - depth), (byte) (blockZ + z1), 0b11,
                            texture
                    };
                }
            }
            */
        }

        return null;
    }

    public static int dir(Face face, byte x, byte y, byte z) {
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
