package boat.world;

import boat.Window;
import boat.util.OpenSimplexNoise;
import org.lwjgl.system.MemoryStack;

public class World {
    public int chunkX = 16;
    public int chunkY = 16;
    public int chunkZ = 16;
    public int totalArea = 0;
    public int totalFaces = 0;
    public Window window;
    public OpenSimplexNoise noise = new OpenSimplexNoise((long) (10000));
    public Chunk[] chunks = new Chunk[4096];

    public World(Window main, boolean generate) {
        this.window = main;
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
        try (MemoryStack stack = MemoryStack.create().push()) {
            for (int x = 0; x < this.chunkX; x++) {
                for (int y = 0; y < this.chunkY; y++) {
                    for (int z = 0; z < this.chunkZ; z++) {
                        this.chunks[(x << 8) | (y << 4) | (z)].buildMesh();
                    }
                }
            }
        }
        System.out.printf("Meshed %d chunks in %d ms\n", this.chunkX * this.chunkY * this.chunkZ, System.currentTimeMillis() - start);
    }

    public void genChunk(int x, int y, int z) {
        this.chunks[(x << 8) | (y << 4) | (z)].generateBlocks();
    }

    public void meshChunk(int x, int y, int z) {
//        this.chunks[(x << 8) | (y << 4) | (z)].buildMesh();
    }

    public void createChunk(int x, int y, int z) {
        this.chunks[(x << 8) | (y << 4) | (z)] = new Chunk(this, x, y, z);
    }

    public Chunk get(int x, int y, int z) {
        return this.chunks[(x << 8) | (y << 4) | (z)];
    }
}
