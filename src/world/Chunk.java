package world;

import block.BlockRegistry;
import block.BlockState;
import org.joml.Vector3i;

import java.util.ArrayList;

import static org.lwjgl.opengl.GL46.*;
import static util.GlHelper.genVertices;

public class Chunk {
    public BlockState[] blocks = new BlockState[4096];
    public int buffer = -1;
    public int triangleCount;

    public Vector3i chunkPos;

    public Chunk(int x, int y, int z) {
        this.chunkPos = new Vector3i(x, y, z);
        for (int i = 0; i < 4096; i ++) {
            blocks[i] = BlockRegistry.registry[(int) (Math.random() * BlockRegistry.totalStates)];
        }
        this.buildMesh();
    }

    public void regen() {
        for (int i = 0; i < 4096; i ++) {
            blocks[i] = BlockRegistry.registry[(int) (Math.random() * BlockRegistry.totalStates)];
        }
        this.buildMesh();
    }

    public BlockState getBlockState(int x, int y, int z) {
        return this.blocks[(x << 8) | (y << 4) | (z)];
    }

    public void setBlockState(BlockState state, int x, int y, int z) {
        this.blocks[(x << 8) | (y << 4) | (z)] = state;
    }

    public void buildMesh() {
        BlockState[] blockStates = this.blocks;
        if (this.buffer == -1) this.buffer = glGenBuffers();

        glBindBuffer(GL_ARRAY_BUFFER, this.buffer);
        ArrayList<short[]> faceArray = new ArrayList<>();
        for (Face face : Face.values()) {
            for (int i = 0; i < blockStates.length; i++) {
                if (!blockStates[i].isFullCube) continue;
                short[] vertices = genVertices(this, face, (short) ((i & 0xf00) >> 8), (short) ((i & 0x0f0) >> 4), (short) (i & 0x00f));
                if (vertices != null) faceArray.add(vertices);
            }
        }

        short[] mesh = new short[faceArray.size() * 18];
        int i = 0;

        for (short[] face : faceArray) {
            for (short vertex : face) {
                mesh[i++] = vertex;
            }
        }
        this.triangleCount = mesh.length / 3;

        glBufferData(GL_ARRAY_BUFFER, mesh, GL_STATIC_DRAW);
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
        EAST,
        WEST,
        UP,
        DOWN,
    }
}
