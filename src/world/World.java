package world;

import java.util.HashMap;

public class World {
    public HashMap<Integer, Chunk> chunks = new HashMap<>();
    public void genChunk(int x, int y, int z) {
        int key = (x << 8) | (y << 4) | (z);
        if (this.chunks.containsKey(key)) throw new Error("Chunk already present");
        this.chunks.put(key, new Chunk(x, y, z));
    }
    public void regenChunk(int x, int y, int z) {
        int key = (x << 8) | (y << 4) | (z);
        this.chunks.get(key).regen();
    }
    public Chunk get(int x, int y, int z) {
        return this.chunks.get((x << 8) | (y << 4) | (z));
    }
}
