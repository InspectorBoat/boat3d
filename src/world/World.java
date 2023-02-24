package world;

import util.OpenSimplexNoise;

public class World {
    public int chunkX = 1;
    public int chunkY = 1;
    public int chunkZ = 1;
    public OpenSimplexNoise noise = new OpenSimplexNoise();
    public Chunk[] chunks = new Chunk[4096];

    public World(boolean generate) {
        for (int x = 0; x < this.chunkX; x++) {
            for (int y = 0; y < this.chunkY; y++) {
                for (int z = 0; z < this.chunkZ; z++) {
                    this.createChunk(x, y, z);
                }
            }
        }

        if (!generate) return;

        long start = System.currentTimeMillis();
        for (int x = 0; x < this.chunkX; x++) {
            for (int y = 0; y < this.chunkY; y++) {
                for (int z = 0; z < this.chunkZ; z++) {
                    this.genChunk(x, y, z);
                }
            }
        }
        System.out.printf("Generated %d chunks in %d ms\n", this.chunkX * this.chunkY * this.chunkZ, System.currentTimeMillis() - start);
        start = System.currentTimeMillis();
        for (int x = 0; x < this.chunkX; x++) {
            for (int y = 0; y < this.chunkY; y++) {
                for (int z = 0; z < this.chunkZ; z++) {
                    this.meshChunk(x, y, z);
                }
            }
        }
        System.out.printf("Meshed %d chunks in %d ms\n", this.chunkX * this.chunkY * this.chunkZ, System.currentTimeMillis() - start);
    }

    public void genChunk(int x, int y, int z) {
        this.chunks[(x << 8) | (y << 4) | (z)].generateBlocks();
    }

    public void meshChunk(int x, int y, int z) {
        this.chunks[(x << 8) | (y << 4) | (z)].buildMesh();
    }

    public void createChunk(int x, int y, int z) {
        this.chunks[(x << 8) | (y << 4) | (z)] = new Chunk(this, x, y, z);
    }

    public Chunk get(int x, int y, int z) {
        return this.chunks[(x << 8) | (y << 4) | (z)];
    }
}
