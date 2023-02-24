package world;

import block.BlockRegistry;
import block.BlockState;
import org.joml.Vector3i;
import org.lwjgl.PointerBuffer;
import org.lwjgl.system.MemoryStack;
import org.lwjgl.system.MemoryUtil;
import util.GlHelper;

import java.nio.ByteBuffer;
import java.nio.IntBuffer;
import java.util.ArrayList;

import static org.lwjgl.opengl.GL46C.*;

public class Chunk {
    public BlockState[] blocks = new BlockState[4096];
    public int buffer = -1;
    public int vertexCount;
    public int[] counts = new int[6];
    public long[] offsets = new long[6];
    public IntBuffer countBuffer = MemoryUtil.memAllocInt(6);
    public PointerBuffer offsetBuffer = MemoryUtil.memAllocPointer(6);
    public World world;
    public boolean doneMeshing;

    public Vector3i chunkPos;

    public Chunk(World world, int x, int y, int z) {
        this.world = world;
        this.chunkPos = new Vector3i(x, y, z);
    }

    public void generateBlocks() {
        int i = 0;
        for (int x = 0; x < 16; x ++) {
            for (int y = 0; y < 16; y++) {
                for (int z = 0; z < 16; z++) {
                    blocks[i] = BlockRegistry.registry[this.world.noise.eval(x / 16d + this.chunkPos.x, y / 16d + this.chunkPos.y, z / 16d + this.chunkPos.z) < 0.1 ? 0 : 1];
//                    blocks[i] = BlockRegistry.registry[1];
                    i++;
                }
            }
        }
//        System.out.printf("Generated chunk in %d ms\n", System.currentTimeMillis() - start);
    }

    // TODO: Fix inner face culling bug
    public void buildMesh() {
        if (this.buffer == -1) this.buffer = glCreateBuffers();
        long start = System.currentTimeMillis();
        BlockState[] blockStates = this.blocks;
        ArrayList<byte[]> faceArray = new ArrayList<>();
        for (Face face : Face.values()) {
            this.offsets[face.ordinal()] = faceArray.size() * 20L;
            int startingFaceArraySize = faceArray.size();
            for (int i = 0; i < blockStates.length; i++) {
                BlockState state = blockStates[i];
                if (!state.hasModel()) continue;
                byte[] vertices = GlHelper.genVertices(this.world, this, state, face, (byte) ((i & 0xf00) >> 8), (byte) ((i & 0x0f0) >> 4), (byte) (i & 0x00f));
                if (vertices != null) faceArray.add(vertices);
            }
            this.counts[face.ordinal()] = (faceArray.size() - startingFaceArraySize) * 5;
        }

        this.vertexCount = faceArray.size() * 16;
        byte[] mesh = new byte[faceArray.size() * 16];
        int i = 0;

        for (byte[] face : faceArray) {
            for (int v = 0; v < 16; ) {
                mesh[i++] = face[v++];
                mesh[i++] = face[v++];
                mesh[i++] = face[v++];
                // Left 14 bits represent texture Id
                // Right 2 represent UV coords
                mesh[i++] = (byte) (face[v++] | face[16]);
            }
        }
        try (MemoryStack stack = MemoryStack.stackPush()) {
            ByteBuffer byteBuffer = stack.malloc(mesh.length);
            byteBuffer.put(0, mesh);
            byteBuffer.position(0);
            glNamedBufferData(this.buffer, byteBuffer, GL_STATIC_DRAW);
        }
        this.countBuffer.put(this.counts);
        this.offsetBuffer.put(this.offsets);
        countBuffer.position(0);
        offsetBuffer.position(0);
        this.doneMeshing = true;
        System.out.printf("Meshed chunk in %d ms\n", System.currentTimeMillis() - start);
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
        NORTH,
        SOUTH,
        WEST,
        EAST,
        DOWN,
        UP,
    }
}
