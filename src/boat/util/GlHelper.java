package boat.util;

import boat.block.BlockState;
import boat.world.Chunk;
import boat.world.Chunk.Face;
import boat.world.World;

import java.nio.IntBuffer;
import java.util.Random;

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

    public static void sortBuffer(IntBuffer buffer, int[] array, int[] counts, int startIndex, int endIndex) {
        for (int i = 0, sum = startIndex; i < counts.length; i++) {
            int prev = counts[i];
            counts[i] = sum;
            sum += prev;
        }
        int pos = startIndex;
        while (pos < endIndex) {
            int n = array[pos];
            int v = buffer.get(pos);
            int dest = counts[n & 0xf];

            if (dest <= pos) {
                if (dest < pos) {
                    buffer.put(pos, buffer.get(dest));
                    buffer.put(dest, v);

                    array[pos] = array[dest];
                    array[dest] = n;
                }
                else ++pos;
                counts[n & 0xf] = dest + 1;
            }
            else ++pos;
        }

    }
}
