package src.world

import src.Window
import src.util.OpenSimplexNoise
import org.lwjgl.system.MemoryStack

class World(var window: Window) {
    @JvmField
    var chunkX = 16
    @JvmField
    var chunkY = 16
    @JvmField
    var chunkZ = 16
    var totalArea = 0
    var totalFaces = 0
    var byteCount = 0
    var noise = OpenSimplexNoise(10000)
    var chunks = arrayOfNulls<Chunk>(4096) as Array<Chunk>
    var totalVolume = 0

    init {
        for (x in 0 until chunkX) {
            for (y in 0 until chunkY) {
                for (z in 0 until chunkZ) {
                    createChunk(x, y, z)
                }
            }
        }

        var start: Long = System.currentTimeMillis();
        for (x in 0 until chunkX) {
            for (y in 0 until chunkY) {
                for (z in 0 until chunkZ) {
                    genChunk(x, y, z)
                }
            }
        }

        println(start - System.currentTimeMillis())
        val warmupRuns = 0
        if (warmupRuns > 0) {
            MemoryStack.create(65536 * Integer.BYTES).push().use { stack ->
                start = System.currentTimeMillis()
                for (i in 0 until warmupRuns) {
                    for (x in 0 until chunkX) {
                        for (y in 0 until chunkY) {
                            for (z in 0 until chunkZ) {
                                chunks[x shl 8 or (y shl 4) or z]!!.buildMesh(stack, true)
                            }
                        }
                    }
                }
            }
            val end = System.currentTimeMillis()
            System.out.printf("cold: %d chunks | %d ms | %.3f ms / chunk\n", chunkX * chunkY * chunkZ * warmupRuns, end - start, (end - start).toDouble() / (chunkX * chunkY * chunkZ * warmupRuns).toDouble())
            noise = OpenSimplexNoise(1)
            for (x in 0 until chunkX) {
                for (y in 0 until chunkY) {
                    for (z in 0 until chunkZ) {
                        genChunk(x, y, z)
                    }
                }
            }
        }
        val runs = 1
        MemoryStack.create(65536 * Integer.BYTES).push().use { stack ->
            start = System.currentTimeMillis()
            for (i in 0 until runs) {
                this.totalFaces = 0
                for (x in 0 until chunkX) {
                    for (y in 0 until chunkY) {
                        for (z in 0 until chunkZ) {
                            this[x, y, z].buildMesh(stack, false)
                        }
                    }
                }
            }
            val end = System.currentTimeMillis()
            System.out.printf("warm: %d chunks | %d ms | %.3f ms / chunk\n", chunkX * chunkY * chunkZ * runs, end - start, (end - start).toDouble() / (chunkX * chunkY * chunkZ * runs).toDouble())
        }
        System.out.printf("%d volume | %d faces | %d area | ratio %.2f%%\n", totalVolume, totalFaces, totalArea, totalFaces.toDouble() / totalArea.toDouble() * 100.0)
        System.out.printf("%d bytes | %d kb | %d mb\n", byteCount, byteCount / 1024, byteCount / 1024 / 1024)
    }

    fun genChunk(x: Int, y: Int, z: Int) {
        chunks[x shl 8 or (y shl 4) or z]!!.generateBlocks()
    }

    fun meshChunk(x: Int, y: Int, z: Int) {
        MemoryStack.create(16384 * Integer.BYTES).push().use { stack -> chunks[x shl 8 or (y shl 4) or z]!!.buildMesh(stack, true) }
    }

    fun createChunk(x: Int, y: Int, z: Int) {
        chunks[x shl 8 or (y shl 4) or z] = Chunk(this, x, y, z)
    }

    operator fun get(x: Int, y: Int, z: Int): Chunk {
        return chunks[x shl 8 or (y shl 4) or z]
    }
    operator fun get(i: Int): Chunk {
        return chunks[i]
    }
}