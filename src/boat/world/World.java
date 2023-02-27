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
    public int byteCount = 0;
    public Window window;
    public OpenSimplexNoise noise = new OpenSimplexNoise(10000);
    public Chunk[] chunks = new Chunk[4096];

    @SuppressWarnings("AutoBoxing")
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
        for (int x = 0; x < this.chunkX; x++) {
            for (int y = 0; y < this.chunkY; y++) {
                for (int z = 0; z < this.chunkZ; z++) {
                    this.genChunk(x, y, z);
                }
            }
        }

        long start;

        int warmupRuns = 0;
        //noinspection ConstantConditions
        if (warmupRuns > 0) {
            try (MemoryStack stack = MemoryStack.create(65536 * Integer.BYTES).push()) {
                start = System.currentTimeMillis();
                for (int i = 0; i < warmupRuns; i ++) {
                    for (int x = 0; x < this.chunkX; x ++) {
                        for (int y = 0; y < this.chunkY; y ++) {
                            for (int z = 0; z < this.chunkZ; z ++) {
                                this.chunks[(x << 8) | (y << 4) | (z)].buildMesh(stack, true);
                            }
                        }
                    }
                }
            }
            long end = System.currentTimeMillis();
            System.out.printf("warmup: %d chunks | %d ms | %.3f ms / chunk\n", this.chunkX * this.chunkY * this.chunkZ * warmupRuns, end - start, (double) (end - start) / (double) (this.chunkX * this.chunkY * this.chunkZ * warmupRuns));
            this.noise = new OpenSimplexNoise(1);
            for (int x = 0; x < this.chunkX; x ++) {
                for (int y = 0; y < this.chunkY; y ++) {
                    for (int z = 0; z < this.chunkZ; z ++) {
                        this.genChunk(x, y, z);
                    }
                }
            }
        }

        int runs = 1;
        try (MemoryStack stack = MemoryStack.create(65536 * Integer.BYTES).push()) {
            start = System.currentTimeMillis();
            for (int i = 0; i < runs; i ++) {
                for (int x = 0; x < this.chunkX; x++) {
                    for (int y = 0; y < this.chunkY; y++) {
                        for (int z = 0; z < this.chunkZ; z++) {
                            this.chunks[(x << 8) | (y << 4) | (z)].buildMesh(stack, false);
                        }
                    }
                }
            }
            long end = System.currentTimeMillis();
            System.out.printf("%d chunks | %d ms | %.3f ms / chunk\n", this.chunkX * this.chunkY * this.chunkZ * runs, end - start, (double) (end - start) / (double) (this.chunkX * this.chunkY * this.chunkZ * runs));
        }
        System.out.printf("%d faces | %d area | ratio %.2f%%\n", this.totalFaces, this.totalArea, (double) (this.totalFaces) / (double) (this.totalArea) * 100d);
        System.out.printf("%d bytes | %d kb | %d mb\n", this.byteCount, this.byteCount / 1024, this.byteCount / 1024 / 1024);
    }

    public void genChunk(int x, int y, int z) {
        this.chunks[(x << 8) | (y << 4) | (z)].generateBlocks();
    }

    public void meshChunk(int x, int y, int z) {
        try (MemoryStack stack = MemoryStack.create(16384 * Integer.BYTES).push()) {
            this.chunks[(x << 8) | (y << 4) | (z)].buildMesh(stack, true);
        }
    }

    public void createChunk(int x, int y, int z) {
        this.chunks[(x << 8) | (y << 4) | (z)] = new Chunk(this, x, y, z);
    }

    public Chunk get(int x, int y, int z) {
        return this.chunks[(x << 8) | (y << 4) | (z)];
    }
}
