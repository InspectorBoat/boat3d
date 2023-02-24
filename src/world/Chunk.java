package world;

import block.BlockRegistry;
import block.BlockState;
import org.joml.Vector3i;
import org.lwjgl.PointerBuffer;
import org.lwjgl.system.MemoryUtil;

import java.nio.IntBuffer;
import java.util.ArrayList;
import java.util.Arrays;

import static org.lwjgl.opengl.GL46C.*;
import static util.GlHelper.dir;

@SuppressWarnings("AutoBoxing")
public class Chunk {
    public BlockState[] blocks = new BlockState[4096];
    public int buffer = -1;
    public int faceCount;
    public int[] counts = new int[6];
    public long[] offsets = new long[6];
    public IntBuffer countBuffer = MemoryUtil.memCallocInt(6);
    public PointerBuffer offsetBuffer = MemoryUtil.memCallocPointer(6);
    public World world;
    public boolean doneMeshing;

    public Vector3i chunkPos;

    public Chunk(World world, int x, int y, int z) {
        this.world = world;
        this.chunkPos = new Vector3i(x, y, z);
    }

    public void generateBlocks() {
        int i = 0, j = 0;
        for (int x = 0; x < 16; x ++) {
            for (int y = 0; y < 16; y++) {
                for (int z = 0; z < 16; z++) {
//                    if (j % 2 == 0) blocks[i] = BlockRegistry.registry[1];
//                    else blocks[i] = BlockRegistry.registry[0];
                    double noiseVal = this.world.noise.eval(x / 16d + this.chunkPos.x, y / 16d + this.chunkPos.y, z / 16d + this.chunkPos.z);
                    blocks[i] = BlockRegistry.registry[noiseVal < 0.1 ? (noiseVal < 0.01 ? 0 : 3) : 1];
//                    blocks[i] = BlockRegistry.registry[1];
                    i++;
                    j++;
                }
                j++;
            }
            j++;
        }
//        System.out.printf("Generated chunk in %d ms\n", System.currentTimeMillis() - start);
    }

    // TODO: Fix inner face culling bug
    public void buildMesh() {
        if (this.buffer == -1) this.buffer = glCreateBuffers();
//        long start = System.currentTimeMillis();
        ArrayList<Integer> faceArray = new ArrayList<>();
        for (Face face : Face.values()) {
            int startIndex = faceArray.size();
            for (int x = 0, i = 0; x < 16; x++) {
                int xOffset = x * 16 << 24;
                for (int y = 0; y < 16; y++) {
                    int yOffset = y * 16 << 16;
                    for (int z = 0; z < 16; z++, i++) {
                        if (this.blocks[i].model == null) continue;

                        int offset = dir(face, (byte) x, (byte) y, (byte) z);
                        if (offset >= 0 && offset <= 4095) {
                            short[] offsetCull = this.blocks[offset].cullProfile(face, true);
                            short[] thisCull = this.blocks[i].cullProfile(face, false);
                            if (thisCull[0] != -1 && offsetCull[0] != -1 &&
                                    offsetCull[0] <= thisCull[0] &&
                                    offsetCull[1] <= thisCull[1] &&
                                    offsetCull[2] >= thisCull[2] &&
                                    offsetCull[3] >= thisCull[3]) {
                                continue;
                            }
                        }
                        else {
                            int chunkOffset = dir(face, (byte) this.chunkPos.x, (byte) this.chunkPos.y, (byte) this.chunkPos.z);
                            if (chunkOffset >= 0 && chunkOffset <= 4095){
                                Chunk chunk = this.world.chunks[chunkOffset];
                                if (chunk != null) {
                                    short[] offsetCull = chunk.blocks[offset - 8192].cullProfile(face, true);
                                    short[] thisCull = this.blocks[i].cullProfile(face, false);
                                    if (thisCull[0] != -1 && offsetCull[0] != -1 &&
                                            offsetCull[0] <= thisCull[0] &&
                                            offsetCull[1] <= thisCull[1] &&
                                            offsetCull[2] >= thisCull[2] &&
                                            offsetCull[3] >= thisCull[3])
                                    {
                                        continue;
                                    }
                                }
                            }
                        }

                        int zOffset = z * 16 << 8;
                        faceArray.add(this.blocks[i].model.faces[face.ordinal()][0] + xOffset + yOffset + zOffset);
                        faceArray.add(this.blocks[i].model.faces[face.ordinal()][1]);
                    }
                }
            }
            this.counts[face.ordinal()] = (faceArray.size() - startIndex) * 5 / 2;
            this.offsets[face.ordinal()] = (startIndex) * 5L / 2 * 4;
        }

        int[] mesh = new int[faceArray.size() + 4];
        mesh[0] = this.chunkPos.x;
        mesh[1] = this.chunkPos.y;
        mesh[2] = this.chunkPos.z;
        for (int i = 0; i < faceArray.size(); i ++) {
            mesh[i + 4] = faceArray.get(i);
        }
        this.faceCount = (mesh.length - 4) / 2;
        glNamedBufferData(this.buffer, mesh, GL_STATIC_DRAW);
        this.countBuffer.put(this.counts);
        this.offsetBuffer.put(this.offsets);
        this.countBuffer.position(0);
        this.offsetBuffer.position(0);
        this.doneMeshing = true;
//        System.out.printf("Meshed chunk in %d ms\n", System.currentTimeMillis() - start);
    }

    public BlockState getBlockState(int x, int y, int z) {
        return this.blocks[(x << 8) | (y << 4) | (z)];
    }

    public void setBlockState(BlockState state, int x, int y, int z) {
        this.blocks[(x << 8) | (y << 4) | (z)] = state;
    }

    public static Vector3i toVec(int i) {
        return new Vector3i(i & 0xf00 >> 8, i & 0x0f0 >> 4, i & 0x00f);
    }

    public static Vector3i toVec(int i, Vector3i vec) {
        return vec.set((i & 0xf00) >> 8, (i & 0x0f0) >> 4, i & 0x00f);
    }

    public static int toIndex(int x, int y, int z) {
        return (x << 8) | (y << 4) | (z);
    }

    public enum Face {
        SOUTH,
        NORTH,
        WEST,
        EAST,
        DOWN,
        UP,
    }
}
