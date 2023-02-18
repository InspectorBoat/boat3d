package world;

public class World {
    public int chunkX = 2;
    public int chunkY = 2;
    public int chunkZ = 2;

    public Chunk[] chunks = new Chunk[4096];

    public World() {
        for (int x = 0; x < chunkX; x++) {
            for (int y = 0; y < chunkY; y++) {
                for (int z = 0; z < chunkZ; z++) {
                    this.genChunk(x, y, z);
                }
            }
        }
    }

    public void genChunk(int x, int y, int z) {
        this.chunks[(x << 8) | (y << 4) | (z)] = new Chunk(x, y, z);
    }

    public void regenChunk(int x, int y, int z) {
        this.chunks[(x << 8) | (y << 4) | (z)].regen();
    }

    public Chunk get(int x, int y, int z) {
        return this.chunks[(x << 8) | (y << 4) | (z)];
    }
}
