@file:Suppress("MemberVisibilityCanBePrivate", "VARIABLE_WITH_REDUNDANT_INITIALIZER")

package src.world

import src.block.BlockFace
import src.block.Blocks
import src.block.BlockState
import src.util.DebugHelper.*
import src.util.Run
import org.joml.Vector3i
import org.lwjgl.PointerBuffer
import org.lwjgl.opengl.GL45C.*
import org.lwjgl.system.MemoryStack
import org.lwjgl.system.MemoryUtil
import java.nio.IntBuffer


class Chunk(var world: World, x: Int, y: Int, z: Int) {
    @JvmField
    var volume: Int = 0
    lateinit var blocks: Array<BlockState>
    @JvmField
    var buffer = glCreateBuffers()
    @JvmField
    var faceCount = 0
    @JvmField
    var counts = IntArray(6)
    @JvmField
    var offsets = LongArray(6)
    @JvmField
    var countBuffer: IntBuffer = MemoryUtil.memCallocInt(6)
    @JvmField
    var offsetBuffer: PointerBuffer = MemoryUtil.memCallocPointer(6)
    @JvmField
    var doneMeshing = false
    @JvmField
    var pos: Vector3i
    @JvmField
    val posIndex: Int
    var area = 0

    init {
        pos = Vector3i(x, y, z)
        posIndex = toIndex(x, y, z)
    }

    operator fun get(x: Int, y: Int, z: Int): BlockState {
        return this.blocks[(x shl 8) or (y shl 4) or (z)]
    }

    operator fun get(i: Int): BlockState {
        return this.blocks[i]
    }

    operator fun set(x: Int, y: Int, z: Int, state: BlockState): BlockState {
        this.blocks[(x shl 8) or (y shl 4) or (z)] = state
        return state
    }

    fun generateBlocks() {
        @Suppress("UNCHECKED_CAST")
        blocks = arrayOfNulls<BlockState>(4096) as Array<BlockState>
        this.world.totalVolume -= this.volume
//        val random = Random(1)
        var i = 0
        var j = 0
        for (x in 0..15) {
            for (y in 0..15) {
                for (z in 0..15) {
//                    blocks[i] = Blocks.states[when (y % 2) {
//                        0 -> 1
//                        else -> 1
//                    }]
//                    blocks[i] = Blocks.states[1]
//                    blocks[i] = BlockRegistry.registry[when (random.nextInt(0, 2)) {
//                        1 -> 1
//                        else -> 0
//                    }]
                    val noise = world.noise.eval(x / 16.0 + pos.x, y / 16.0 + pos.y, z / 16.0 + pos.z)
                    this[x, y, z] = Blocks.states[when ((noise * 12 + 1).toInt()) {
                        1, 2, 3 -> {
                            this.volume ++
                            1
                        }
                        else -> 0
                    }]
                    i++
                    j++
                }
                j++
            }
            j++
        }
        this.world.totalVolume += this.volume
//        logn(this.volume)
    }

    fun buildMesh(stack: MemoryStack, mock: Boolean) {
        if (buffer == -1) buffer = glCreateBuffers()
        area = 0
        faceCount = 0

        stack.push().use { st ->
            val buffer = st.mallocInt(st.pointer / 4)
            val row = Array(16) {Run()}
            meshSouth(buffer, row)

            if (mock) return
            faceCount = buffer.position() / 2
            buffer.limit(buffer.position())
            buffer.position( 0)
            world.totalArea += area
            world.totalFaces += faceCount
            world.byteCount += faceCount * 4
            glNamedBufferStorage(this.buffer, (buffer.limit() + 4L) * Integer.BYTES, GL_DYNAMIC_STORAGE_BIT or GL_MAP_WRITE_BIT)
            glNamedBufferSubData(this.buffer, 0, intArrayOf(pos.x, pos.y, pos.z))
            glNamedBufferSubData(this.buffer, (4 * Integer.BYTES).toLong(), buffer)
        }
        countBuffer.put(counts)
        offsetBuffer.put(offsets)
        countBuffer.position(0)
        offsetBuffer.position(0)
        doneMeshing = true
    }

    fun meshSouth(buffer: IntBuffer, row: Array<Run>) {
        val startPos = buffer.position()
        offsets[Norm.SOUTH.ordinal] = buffer.position() * 5L / 2 * Integer.BYTES
        nolog()
        var expectedArea = 0
        var face: BlockFace? = null
        var run: Run = row[0]
        var activeRun = false
        var sameRow = false

        out@
        for (z in 0..15) {
            for (y in 0..15) {
                for (x in 0..15) {
                    val pos = toIndex(x, y, z)
                    if (!blocks[pos].hasModel) {
                        activeRun = false
                        continue
                    }
                    face = blocks[pos].model[Norm.SOUTH]
                    if (isFaceCulled(face, x, y, z, pos, -Z, NORTH)) {
//                        if (activeRun) logf("\n%02d %02d %02d | end active run", x, y, z)
                        activeRun = false
                        continue
                    }
                    expectedArea++

//                    logf("\n%02d %02d %02d | ", x, y, z)
//                    if (false)
                    if (activeRun && sameRow) {
                        if (matchRightEdge(run, face)) {
                            mergeFace(buffer, run, face)
//                            logf("extend %02d %02d %02d", run.start, y, z)
                            continue
                        }
                        activeRun = false
                    }
//                    if (false)
                    if (!activeRun) {
                        if (row[x].row == (z * 256 + y - 1) && matchLeftBottom(row[x], face)) {
                            sameRow = false
                            activeRun = true
                            run = row[x]
//                            logf("begin merge: %02d-%02d %02d %02d : ", run.start, run.end, y - 1, z)
                        }
                    }
//                    if (false)
                    if (activeRun) {
                        if (run.end != x) {
                            val nextPos = pos + X
                            if (!blocks[nextPos].hasModel ||
                                    !matchLeftBottomEdge(face, blocks[nextPos].model[SOUTH]) ||
                                    isFaceCulled(blocks[nextPos].model[SOUTH], x + 1, y, z, nextPos, -Z, NORTH)) {
//                                logf("abort merge: missing face")
                                pullPartialRun(buffer, run, face, x, y, z)
                                activeRun = false
                            }
                            else logf("continue merge")
                        } else {
                            if (matchRightBottom(run, face)) {
//                                logf("finish merge: merge success")
                                pullRun(buffer, run, face)
                                activeRun = false
                            } else {
//                                logf("finish merge: merge failed")
                                pullPartialRun(buffer, run, face, x, y, z)
                                sameRow = true
                            }
                        }
                        continue
                    }

                    logf("begin new run")
                    run = row[x]
                    sameRow = true
                    activeRun = true
                    newRun(buffer, run, face, x, y, z)
                }
//                logf("\nnext row")
                activeRun = false
            }
        }
//        logfn("\nGenerated area %d | Expected area %d", area, expectedArea)
        counts[SOUTH] = (buffer.position() - startPos) * 5 / 2
    }

    /**
     * Extends the active run in the current row
     */
    fun mergeFace(buffer: IntBuffer, run: Run, face: BlockFace) {
        buffer.put(run.pointer, buffer[run.pointer] + ((face.w + 1) shl 24))
        run.end ++
        run.maxY = face.y + face.h
        area ++
    }

    /**
     * Buffers a new run ranging from minX/minY of the run to the maxX/maxY of the end face
     * Used when a merge with run in the last row was aborted
     */
    fun pullPartialRun(buffer: IntBuffer, run: Run, face: BlockFace, x: Int, y: Int, z: Int): Run {
        buffer.put(((run.start * 16 + run.minX) shl 24) + ((y * 16) shl 16) + ((z * 16) shl 8))
        run.pointer = buffer.position()
        buffer.put(face.material + (16 * (x - run.start)).shl(24))

        run.end     = x
        run.row     = z * 256 + y
        area       += x - run.start + 1

        return run
    }

    /**
     * Extends the run into the current row
     * After pulling the run up, we can never match more faces in the same row
     * So this returns nothing
     */
    fun pullRun(buffer: IntBuffer, run: Run, face: BlockFace) {
        buffer.put(run.pointer, buffer[run.pointer] + ((face.h + 1) shl 16))

        run.maxY = face.x + face.w
        run.row ++
        area += run.end - run.start + 1
    }

    /**
     * Buffers a new run from the provided face
     * Returns the new active run
     */
    fun newRun(buffer: IntBuffer, run: Run, face: BlockFace, x: Int, y: Int, z: Int) {

        buffer.put(face.vertex + (x shl 24 shl 4) + (y shl 16 shl 4) + (z shl 8 shl 4))
        run.pointer = buffer.position()
        buffer.put(face.material)

        run.start = x
        run.end = x
        run.texture = face.t

        run.minX = face.x
        run.minY = face.y
        run.maxX = face.x + face.w
        run.maxY = face.y + face.h

        run.row = z * 256 + y

        area ++
    }

    fun isFaceCulled(face: BlockFace, x: Int, y: Int, z: Int, pos: Int, dir: Int, norm: Int): Boolean {
//        return false
        val cullFace: BlockFace =
                if (z > 0 && this[pos + dir].hasModel) {
                    this[pos + dir].model[SOUTH]
                } else if (this.pos.z > 0 && this.world[this.posIndex - Z][pos or 0x00f].hasModel) {
                    this.world[this.posIndex - Z][pos or 0x00f].model[norm]
                } else return false
        return face.x <= cullFace.x &&
                face.y <= cullFace.y &&
                face.x + face.w >= cullFace.x + cullFace.w &&
                face.y + face.h >= cullFace.y + cullFace.h
    }

    companion object {
        /**
         * Checks if the current face can merge with the active run in the same row
         * Requires minX of the current face to be 0
         * Requires maxX of the current run to be 15
         * Requires minY of the current face to equal minY of the current face
         * Requires minY of the current face to equal minY of the current face
         */
        fun matchRightEdge(run: Run, face: BlockFace): Boolean {
//            println(row.texture(i) == face.texture.toInt())
//            println(row.maxX(i) == 15)
//            println(face.x.toInt() == 0)
//            println(row.minY(i) == face.y.toInt())
//            println(row.maxY(i) - row.minY(i) == face.height.toInt())
            return run.texture == face.t &&
                    run.maxX == 15 &&
                    face.x == 0 &&
                    run.minY == face.y &&
                    run.maxY - run.minY == face.h
        }

        /**
         * Checks if the current face can begin merging with the run in the last row
         * Requires minX of the current face and run to be equal
         * Requires minY of the current face to be 0
         * Requires maxY of the run to be 15
         */
        fun matchLeftBottom(row: Run, face: BlockFace): Boolean {
            return row.texture == face.t &&
                    row.maxY == 15 &&
                    face.y == 0 &&
                    row.minX == face.x
        }

        /**
         * Checks if the current face can merge with the next face
         * Requires maxX of the current face to be 15
         * Requires minX of the next face to be 0
         * Requires minY of the next face to be 0
         * Requires maxY of the current face to be equal maxY of the next face
         */
        fun matchLeftBottomEdge(face: BlockFace, next: BlockFace): Boolean {
            return face.t == next.t &&
                    face.x + face.w == 15 &&
                    next.x == 0 &&
                    face.y == next.y &&
                    face.h == next.h
        }

        /**
         * Checks if the current face can finish merging with the run in the last row
         * Requires maxX of the current face and run to be equal
         * Texture comparison is unnecessary
         */
        fun matchRightBottom(run: Run, face: BlockFace): Boolean {
            return face.y == 0 &&
                    face.x + face.w == run.maxX
        }

        fun toIndex(x: Int, y: Int, z: Int): Int {
            return (x shl 8) or (y shl 4) or z
        }

        const val Z: Int = (0 shl 8) or (0 shl 4) or (1 shl 0)
        const val X: Int = (1 shl 8) or (0 shl 4) or (0 shl 0)
        const val Y: Int = (0 shl 8) or (1 shl 4) or (0 shl 0)

        const val SOUTH = 0
        const val NORTH = 1
        const val WEST =  2
        const val EAST =  3
        const val DOWN =  4
        const val UP =    5
    }

    enum class Norm {
        SOUTH, NORTH, WEST, EAST, DOWN, UP
    }

}