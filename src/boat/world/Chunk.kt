@file:Suppress("MemberVisibilityCanBePrivate")

package boat.world

import boat.block.BlockModel
import boat.block.BlockRegistry
import boat.block.BlockState
import boat.util.DebugHelper.*
import boat.util.QuadRun
import boat.util.dir
import org.joml.Vector3i
import org.lwjgl.opengl.GL30C
import org.lwjgl.opengl.GL44C
import org.lwjgl.opengl.GL45C.*
import org.lwjgl.system.MemoryStack
import org.lwjgl.system.MemoryUtil
import java.nio.IntBuffer
import java.util.*


class Chunk(var world: World, x: Int, y: Int, z: Int) {
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
    var countBuffer = MemoryUtil.memCallocInt(6)
    @JvmField
    var offsetBuffer = MemoryUtil.memCallocPointer(6)
    @JvmField
    var doneMeshing = false
    @JvmField
    var chunkPos: Vector3i
    private var area = 0

    init {
        chunkPos = Vector3i(x, y, z)
    }

    fun generateBlocks() {
        @Suppress("UNCHECKED_CAST")
        blocks = arrayOfNulls<BlockState>(4096) as Array<BlockState>
        val random = Random(1)
        var i = 0
        var j = 0
        for (x in 0..15) {
            for (y in 0..15) {
                for (z in 0..15) {
//                    blocks[i] = BlockRegistry.registry[1]
//                    blocks[i] = BlockRegistry.registry[when (random.nextInt(0, 2)) {
//                        1 -> 1
//                        else -> 0
//                    }]
                    val noiseVal = world.noise.eval(x / 16.0 + chunkPos.x, y / 16.0 + chunkPos.y, z / 16.0 + chunkPos.z)
                    blocks[i] = BlockRegistry.registry[when ((noiseVal * 12 + 1).toInt()) {
                        1, 2 -> 0
                        3, 4 -> 1
                        5 -> 1
                        6 -> 1
                        else -> 0
                    }]
                    i++
                    j++
                }
                j++
            }
            j++
        }
    }

    fun buildMesh(stack: MemoryStack, mock: Boolean) {
        if (buffer == -1) buffer = glCreateBuffers()
        area = 0
        faceCount = 0

        val nextRow = Array(16) { QuadRun(IntArray(8)) }
        val thisRow = Array(16) { QuadRun(IntArray(8)) }

        stack.push().use { st ->
            val faceBuffer = st.mallocInt(st.pointer / 4)
            meshSouth(faceBuffer, thisRow, nextRow)
//            val nextRow = Array(16) { IntArray(7) }
//            val thisRow = Array(16) { IntArray(7) }
//
//            this.meshNorth(faceBuffer, thisRow, nextRow);
//            this.meshWest(faceBuffer, thisRow, nextRow);
//            this.meshEast(faceBuffer, thisRow, nextRow);
//            this.meshDown(faceBuffer, thisRow, nextRow);
//            this.meshUp(faceBuffer, thisRow, nextRow);
            if (mock) return
            faceCount = faceBuffer.position() / 2
            faceBuffer.limit(faceBuffer.position())
            faceBuffer.position(0)
            world.totalArea += area
            world.totalFaces += faceCount
            world.byteCount += faceCount * 4
            glNamedBufferStorage(buffer, (faceBuffer.limit() + 4L) * Integer.BYTES, GL44C.GL_DYNAMIC_STORAGE_BIT or GL30C.GL_MAP_WRITE_BIT)
            glNamedBufferSubData(buffer, 0, intArrayOf(chunkPos.x, chunkPos.y, chunkPos.z))
            glNamedBufferSubData(buffer, (4 * Integer.BYTES).toLong(), faceBuffer)
        }
        countBuffer.put(counts)
        offsetBuffer.put(offsets)
        countBuffer.position(0)
        offsetBuffer.position(0)
        doneMeshing = true
    }

    @Suppress("MemberVisibilityCanBePrivate")
    fun meshSouth(faceBuffer: IntBuffer, rowBuffer: Array<QuadRun>, otherRowBuffer: Array<QuadRun>) {
        var currentRow = rowBuffer
        var lastRow = otherRowBuffer
        nolog()
        offsets[Face.SOUTH.ordinal] = faceBuffer.position() * 5L / 2 * Integer.BYTES
        val startPos = faceBuffer.position()
        var activeRun: QuadRun? = null
        var isSameRow = false
        var currentFace: IntArray? = null
        var expectedArea = 0
        for (z in 0..15) {
            for (y in 0..15) {
                for (x in 0..15) {
                    val i = toIndex(x, y, z)

                    //Immediately stop if block lacks a model or is culled
                    if (blocks[i].model == null || isFaceCulled(Face.SOUTH, x, y, z, i)) {
                        if (activeRun != null) {
                            log("\n%02d %02d %02d | end active run", x, y, z)
                        }
                        activeRun = null
                        continue
                    }
                    else {
                        expectedArea++
                    }

                    log("\n%02d %02d %02d | ", x, y, z)

                    currentFace = blocks[i].model.faces[Face.SOUTH.ordinal]

                    //Try to merge to active run in current row
                    if (activeRun != null && isSameRow) {
                        if (canDoWidthMerge(activeRun, currentFace)) {
                            extendRunWidth(faceBuffer, activeRun, currentFace)
                            log("extend %02d %02d %02d", activeRun.startIndex, y, z)
                            continue
                        }
                        activeRun = null
                    }

                    //Try to start merge with run in last row
                    if (activeRun == null) {
                        if (lastRow[x].rowIndex == (z * 256 + y - 1) && canStartHeightMerge(lastRow[x], currentFace)) {
                            activeRun = lastRow[x]
                            isSameRow = false
                            log("begin merge : at %02d %02d %02d : length %02d ", x, y - 1, z, activeRun.endIndex - activeRun.startIndex + 1)
                        }
                    }

                    if (activeRun != null) {
                        //Check if merge is still possible with next face
                        if (activeRun.endIndex != x) {
                            val next = toIndex(x + 1, y, z)
                            if (blocks[next].model == null || isFaceCulled(Face.SOUTH, x + 1, y, z, next)) {
                                log("abort merge: missing face")
                                startNewExtendedRun(faceBuffer, currentRow, activeRun, currentFace, activeRun.startIndex, x, y, z)
                                activeRun = null
                            } else if (!canContinueHeightMerge(currentFace, blocks[next].model.faces[Face.SOUTH.ordinal])) {
                                log("abort merge: mismatched face")
                                startNewExtendedRun(faceBuffer, currentRow, activeRun, currentFace, activeRun.startIndex, x, y, z)
                                activeRun = null
                            } else {
                                log("continue merge", activeRun.startIndex, y - 1, z)
                            }
                        } else {
                            if (canFinishHeightMerge(activeRun, currentFace)) {
                                log("finish merge: merge success")
                                extendRunHeight(faceBuffer, activeRun, currentFace, currentRow)
                                activeRun = null
                            } else {
                                log("finish merge: merge failed")
                                activeRun = startNewExtendedRun(faceBuffer, currentRow, activeRun, currentFace, activeRun.startIndex, x, y, z)
                                isSameRow = true
                            }
                        }
                        continue
                    }

                    //Start a new run if no merges are possible
                    log("begin new run", x, y, z)
                    activeRun = startNewRun(faceBuffer, currentRow, currentFace, x, y, z)
                    isSameRow = true
                }
                val temp = lastRow
                lastRow = currentRow
                currentRow = temp
                activeRun = null
            }
        }
        logn("\nGenerated area %d | Expected area %d", area, expectedArea)
        assert(area == expectedArea)
        counts[Face.SOUTH.ordinal] = (faceBuffer.position() - startPos) * 5 / 2
    }

    /**
     * Extends the active run in the current row
     */
    private fun extendRunWidth(faceBuffer: IntBuffer, runToExtend: QuadRun, face: IntArray) {
        faceBuffer.put(runToExtend.bufferPos, faceBuffer[runToExtend.bufferPos] + (BlockModel.width(face) + 1 shl 24))
        runToExtend.endIndex ++
        runToExtend.endVec = face[0]
        runToExtend.endTex = face[1]
        area ++
    }

    /**
     * Extends the active run in the last row
     * Copies the run information into the current row
     */
    private fun extendRunHeight(faceBuffer: IntBuffer, runToExtend: QuadRun, face: IntArray, currentRow: Array<QuadRun>) {
        faceBuffer.put(runToExtend.bufferPos, faceBuffer[runToExtend.bufferPos] + (BlockModel.height(face) + 1 shl 16))
        val activeRun = currentRow[runToExtend.startIndex]
        activeRun.startVec = runToExtend.startVec
        activeRun.startTex = runToExtend.startTex
        activeRun.startIndex = runToExtend.startIndex
        activeRun.endIndex = runToExtend.endIndex
        activeRun.endVec = face[0]
        activeRun.endTex = face[1]
        activeRun.bufferPos = runToExtend.bufferPos
        activeRun.rowIndex = runToExtend.rowIndex + 1;
        area += runToExtend.endIndex - runToExtend.startIndex + 1
    }

    /**
     * Buffers a new run from the provided face
     * Returns the new active run
     */
    private fun startNewRun(faceBuffer: IntBuffer, currentRow: Array<QuadRun>, face: IntArray, x: Int, y: Int, z: Int): QuadRun {
        faceBuffer.put(face[0] + (x * 16 shl 24) + (y * 16 shl 16) + (z * 16 shl 8))
        faceBuffer.put(face[1])
        val activeRun = currentRow[x]
        activeRun.startVec = face[0]
        activeRun.startTex = face[1]
        activeRun.startIndex = x
        activeRun.endIndex = x
        activeRun.endVec = face[0]
        activeRun.endTex = face[1]
        activeRun.bufferPos = faceBuffer.position() - 1
        activeRun.rowIndex = z * 256 + y
        area++
        return activeRun
    }

    /**
     * Buffers a new run ranging from minX/minY of the first face to the maxX/maxY of the end face
     * Returns the new active run
     */
    private fun startNewExtendedRun(faceBuffer: IntBuffer, currentRow: Array<QuadRun>, startRun: QuadRun, endFace: IntArray, startX: Int, endX: Int, y: Int, z: Int): QuadRun {
        faceBuffer.put(startRun.startVec + (startX * 16 shl 24) + (y * 16 shl 16) + (z * 16 shl 8))
        faceBuffer.put(BlockModel.faceTexture(startRun.startWidth + 1 + 16 * (endX - startX - 1) + BlockModel.width(endFace), BlockModel.height(endFace), BlockModel.texture(endFace)))
        val activeRun = currentRow[startX]
        activeRun.startVec = startRun.startVec
        activeRun.startTex = startRun.startTex
        activeRun.startIndex = startX
        activeRun.endIndex = endX
        activeRun.endVec = endFace[0]
        activeRun.endTex = endFace[1]
        activeRun.bufferPos = faceBuffer.position() - 1
        activeRun.rowIndex = z * 256 + y
        area += endX - startX + 1
        return activeRun
    }

    fun isFaceCulled(face: Face, x: Int, y: Int, z: Int, i: Int): Boolean {
//        if (true) return false;
        val offset = dir(face, x.toByte(), y.toByte(), z.toByte())
        if (offset in 0..4095) {
            val offsetCull = blocks[offset].cullProfile(face, true)
            val thisCull = blocks[i].cullProfile(face, false)
            return thisCull[0].toInt() != -1 && offsetCull[0].toInt() != -1 && offsetCull[0] <= thisCull[0] && offsetCull[1] <= thisCull[1] && offsetCull[2] >= thisCull[2] && offsetCull[3] >= thisCull[3]
        }
        val chunkOffset = dir(face, chunkPos.x.toByte(), chunkPos.y.toByte(), chunkPos.z.toByte())
        if (chunkOffset in 0..4095 && world.chunks[chunkOffset] != null) {
            val offsetCull = world.chunks[chunkOffset].blocks[offset - 8192].cullProfile(face, true)
            val thisCull = blocks[i].cullProfile(face, false)

            return thisCull[0].toInt() != -1 && offsetCull[0].toInt() != -1 && offsetCull[0] <= thisCull[0] && offsetCull[1] <= thisCull[1] && offsetCull[2] >= thisCull[2] && offsetCull[3] >= thisCull[3]
        }
        return false
    }

    fun meshNorth(faceBuffer: IntBuffer, currentRow: Array<IntArray>, lastRow: Array<IntArray>) {
        var currentRow = currentRow
        var lastRow = lastRow
        offsets[Face.NORTH.ordinal] = faceBuffer.position() * 5L / 2 * Integer.BYTES
        val startPos = faceBuffer.position()
        var runIndex = 0
        var activeRun = false
        for (z in 0..15) {
            val zOffset = z * 16 shl 8
            for (x in 0..15) {
                val xOffset = x * 16 shl 24
                var y = 0
                while (y < 16) {
                    val yOffset = y * 16 shl 16
                    var i = toIndex(x, y, z)
                    if (blocks[i].model == null || isFaceCulled(Face.NORTH, x, y, z, i)) {
                        runIndex = y
                        activeRun = false
                        y++
                        continue
                    }
                    val currentFace = blocks[i].model.faces[Face.NORTH.ordinal]
                    //Check if face can be merged with run in current row
                    if (activeRun) {
                        val runToMatch = currentRow[runIndex]
                        if (canMerge(true, currentFace, runToMatch)) {
                            //Add onto the last quad
                            faceBuffer.put(runToMatch[3], faceBuffer[runToMatch[3]] + 0x00100000)

                            //Increment merge run length
                            runToMatch[2] = y
                            area++
                            y++
                            continue
                        }
                    }

                    //Try to merge with run from previous row
                    val runToMatch = lastRow[y]
                    if (canMerge(false, currentFace, runToMatch)) {
                        val startY = y
                        //Index where the previous run ended
                        val runEnd = runToMatch[2]
                        //If run length 1, this doesn't trigger at all. This should never roll over
                        while (++y <= runEnd) {
                            i = x shl 8 or (y shl 4) or z
                            if (blocks[i].model == null) break
                            if (isFaceCulled(Face.NORTH, x, y, z, i)) break
                            val nextFace = blocks[i].model.faces[Face.NORTH.ordinal]
                            if (!canMerge(false, currentFace, runToMatch)) {
                                break
                            }
                        }
                        activeRun = false
                        currentRow[startY][0] = runToMatch[0]
                        currentRow[startY][1] = runToMatch[1]
                        currentRow[startY][2] = --y
                        if (y == runEnd) {
                            faceBuffer.put(runToMatch[3], faceBuffer[runToMatch[3]] + 0x10000000)
                            currentRow[startY][3] = runToMatch[3]
                        } else {
                            faceBuffer.put(currentFace[0] + xOffset + yOffset + zOffset)
                            faceBuffer.put(currentFace[1] + 0x00100000 * (y - startY))
                            currentRow[startY][3] = faceBuffer.position() - 1
                        }
                        area += y - startY
                        y++
                        continue
                    }
                    faceBuffer.put(currentFace[0] + xOffset + yOffset + zOffset)
                    faceBuffer.put(currentFace[1])
                    //Add merge info entry
                    runIndex = y
                    activeRun = true
                    currentRow[y][0] = currentFace[0]
                    currentRow[y][1] = currentFace[1]
                    currentRow[y][2] = y
                    currentRow[y][3] = faceBuffer.position() - 1
                    area++
                    y++
                }
                val temp = lastRow
                lastRow = currentRow
                currentRow = temp
                runIndex = 0
                clearRow(currentRow)
            }
            clearRow(lastRow)
        }
        counts[Face.NORTH.ordinal] = (faceBuffer.position() - startPos) * 5 / 2
    }

    fun meshWest(faceBuffer: IntBuffer, currentRow: Array<IntArray>, lastRow: Array<IntArray>) {
        var currentRow = currentRow
        var lastRow = lastRow
        offsets[Face.WEST.ordinal] = faceBuffer.position() * 5L / 2 * Integer.BYTES
        val startPos = faceBuffer.position()
        var runIndex = 0
        for (x in 0..15) {
            val xOffset = x * 16 shl 24
            for (y in 0..15) {
                val yOffset = y * 16 shl 16
                var z = 0
                while (z < 16) {
                    val zOffset = z * 16 shl 8
                    val i = toIndex(x, y, z)
                    if (blocks[i].model == null || isFaceCulled(Face.WEST, x, y, z, i)) {
                        runIndex = z
                        z++
                        continue
                    }
                    val currentFace = blocks[i].model.faces[Face.WEST.ordinal]
                    var runToMatch = currentRow[runIndex]
                    if (canMerge(true, currentFace, runToMatch)) {
                        faceBuffer.put(runToMatch[3], faceBuffer[runToMatch[3]] + 0x10000000)
                        runToMatch[2] = z
                        area++
                        z++
                        continue
                    }
                    runToMatch = lastRow[z]
                    if (canMerge(true, currentFace, runToMatch)) {
                        val startZ = z
                        val runEnd = runToMatch[2]
                        while (++z <= runEnd) {
                            val nextI = x shl 8 or (y shl 4) or z
                            if (blocks[nextI].model == null) {
                                break
                            }
                            if (isFaceCulled(Face.WEST, x, y, z, nextI)) {
                                break
                            }
                            val nextFace = blocks[nextI].model.faces[Face.WEST.ordinal]
                            if (!canMerge(false, currentFace, runToMatch)) {
                                break
                            }
                        }
                        runIndex = z--
                        currentRow[startZ][0] = runToMatch[0]
                        currentRow[startZ][1] = runToMatch[1]
                        currentRow[startZ][2] = z
                        if (z == runEnd) {
                            faceBuffer.put(runToMatch[3], faceBuffer[runToMatch[3]] + 0x00100000)
                            currentRow[startZ][3] = runToMatch[3]
                        } else {
                            faceBuffer.put(currentFace[0] + xOffset + yOffset + zOffset)
                            faceBuffer.put(currentFace[1] + 0x10000000 * (z - startZ))
                            currentRow[startZ][3] = faceBuffer.position() - 1
                        }
                        area += z - startZ
                    } else {
                        faceBuffer.put(currentFace[0] + xOffset + yOffset + zOffset)
                        faceBuffer.put(currentFace[1])
                        runIndex = z
                        currentRow[z][0] = currentFace[0]
                        currentRow[z][1] = currentFace[1]
                        currentRow[z][2] = z
                        currentRow[z][3] = faceBuffer.position() - 1
                        area++
                    }
                    z++
                }
                val temp = lastRow
                lastRow = currentRow
                currentRow = temp
                runIndex = 0
                clearRow(currentRow)
            }
            clearRow(lastRow)
        }
        counts[Face.WEST.ordinal] = (faceBuffer.position() - startPos) * 5 / 2
    }

    fun meshEast(faceBuffer: IntBuffer, currentRow: Array<IntArray>, lastRow: Array<IntArray>) {
        var currentRow = currentRow
        var lastRow = lastRow
        offsets[Face.EAST.ordinal] = faceBuffer.position() * 5L / 2 * Integer.BYTES
        val startPos = faceBuffer.position()
        var runIndex = 0
        for (x in 0..15) {
            val xOffset = x * 16 shl 24
            for (y in 0..15) {
                val yOffset = y * 16 shl 16
                var z = 0
                while (z < 16) {
                    val zOffset = z * 16 shl 8
                    val i = toIndex(x, y, z)
                    if (blocks[i].model == null || isFaceCulled(Face.EAST, x, y, z, i)) {
                        runIndex = z
                        z++
                        continue
                    }
                    val currentFace = blocks[i].model.faces[Face.EAST.ordinal]
                    var runToMatch = currentRow[runIndex]
                    if (canMerge(true, currentFace, runToMatch)) {
                        faceBuffer.put(runToMatch[3], faceBuffer[runToMatch[3]] + 0x10000000)
                        runToMatch[2] = z
                        area++
                        z++
                        continue
                    }
                    runToMatch = lastRow[z]
                    if (canMerge(false, currentFace, runToMatch)) {
                        val startZ = z
                        val runEnd = runToMatch[2]
                        while (++z <= runEnd) {
                            val nextI = x shl 8 or (y shl 4) or z
                            if (blocks[nextI].model == null) {
                                break
                            }
                            if (isFaceCulled(Face.EAST, x, y, z, nextI)) {
                                break
                            }
                            val nextFace = blocks[nextI].model.faces[Face.EAST.ordinal]
                            if (!canMerge(false, currentFace, runToMatch)) {
                                break
                            }
                        }
                        runIndex = z--
                        currentRow[startZ][0] = runToMatch[0]
                        currentRow[startZ][1] = runToMatch[1]
                        currentRow[startZ][2] = z
                        if (z == runEnd) {
                            faceBuffer.put(runToMatch[3], faceBuffer[runToMatch[3]] + 0x00100000)
                            currentRow[startZ][3] = runToMatch[3]
                        } else {
                            faceBuffer.put(currentFace[0] + xOffset + yOffset + zOffset)
                            faceBuffer.put(currentFace[1] + 0x10000000 * (z - startZ))
                            currentRow[startZ][3] = faceBuffer.position() - 1
                        }
                        area += z - startZ
                    } else {
                        faceBuffer.put(currentFace[0] + xOffset + yOffset + zOffset)
                        faceBuffer.put(currentFace[1])
                        runIndex = z
                        currentRow[z][0] = currentFace[0]
                        currentRow[z][1] = currentFace[1]
                        currentRow[z][2] = z
                        currentRow[z][3] = faceBuffer.position() - 1
                        area++
                    }
                    z++
                }
                val temp = lastRow
                lastRow = currentRow
                currentRow = temp
                runIndex = 0
                clearRow(currentRow)
            }
            clearRow(lastRow)
        }
        counts[Face.EAST.ordinal] = (faceBuffer.position() - startPos) * 5 / 2
    }

    fun meshDown(faceBuffer: IntBuffer, currentRow: Array<IntArray>, lastRow: Array<IntArray>) {
        var currentRow = currentRow
        var lastRow = lastRow
        offsets[Face.DOWN.ordinal] = faceBuffer.position() * 5L / 2 * Integer.BYTES
        val startPos = faceBuffer.position()
        var runIndex = 0
        for (y in 0..15) {
            val yOffset = y * 16 shl 16
            for (x in 0..15) {
                val xOffset = x * 16 shl 24
                var z = 0
                while (z < 16) {
                    val zOffset = z * 16 shl 8
                    val i = toIndex(x, y, z)
                    if (blocks[i].model == null || isFaceCulled(Face.DOWN, x, y, z, i)) {
                        runIndex = z
                        z++
                        continue
                    }
                    val currentFace = blocks[i].model.faces[Face.DOWN.ordinal]
                    var runToMatch = currentRow[runIndex]
                    if (canMerge(true, currentFace, runToMatch)) {
                        faceBuffer.put(runToMatch[3], faceBuffer[runToMatch[3]] + 0x00100000)
                        runToMatch[2] = z
                        area++
                        z++
                        continue
                    }
                    runToMatch = lastRow[z]
                    if (canMerge(false, currentFace, runToMatch)) {
                        val startZ = z
                        val runEnd = runToMatch[2]
                        while (++z <= runEnd) {
                            val nextI = x shl 8 or (y shl 4) or z
                            if (blocks[nextI].model == null) {
                                break
                            }
                            if (isFaceCulled(Face.DOWN, x, y, z, nextI)) {
                                break
                            }
                            val nextFace = blocks[nextI].model.faces[Face.DOWN.ordinal]
                            if (!canMerge(false, currentFace, runToMatch)) {
                                break
                            }
                        }
                        runIndex = z--
                        currentRow[startZ][0] = runToMatch[0]
                        currentRow[startZ][1] = runToMatch[1]
                        currentRow[startZ][2] = z
                        if (z == runEnd) {
                            faceBuffer.put(runToMatch[3], faceBuffer[runToMatch[3]] + 0x10000000)
                            currentRow[startZ][3] = runToMatch[3]
                        } else {
                            faceBuffer.put(currentFace[0] + xOffset + yOffset + zOffset)
                            faceBuffer.put(currentFace[1] + 0x00100000 * (z - startZ))
                            currentRow[startZ][3] = faceBuffer.position() - 1
                        }
                        area += z - startZ
                    } else {
                        faceBuffer.put(currentFace[0] + xOffset + yOffset + zOffset)
                        faceBuffer.put(currentFace[1])
                        runIndex = z
                        currentRow[z][0] = currentFace[0]
                        currentRow[z][1] = currentFace[1]
                        currentRow[z][2] = z
                        currentRow[z][3] = faceBuffer.position() - 1
                        area++
                    }
                    z++
                }
                val temp = lastRow
                lastRow = currentRow
                currentRow = temp
                runIndex = 0
                clearRow(currentRow)
            }
            clearRow(lastRow)
        }
        counts[Face.DOWN.ordinal] = (faceBuffer.position() - startPos) * 5 / 2
    }

    fun meshUp(faceBuffer: IntBuffer, currentRow: Array<IntArray>, lastRow: Array<IntArray>) {
        var currentRow = currentRow
        var lastRow = lastRow
        offsets[Face.UP.ordinal] = faceBuffer.position() * 5L / 2 * Integer.BYTES
        val startPos = faceBuffer.position()
        var runIndex = 0
        for (y in 0..15) {
            val yOffset = y * 16 shl 16
            for (x in 0..15) {
                val xOffset = x * 16 shl 24
                var z = 0
                while (z < 16) {
                    val zOffset = z * 16 shl 8
                    val i = toIndex(x, y, z)
                    if (blocks[i].model == null || isFaceCulled(Face.UP, x, y, z, i)) {
                        runIndex = z
                        z++
                        continue
                    }
                    val currentFace = blocks[i].model.faces[Face.UP.ordinal]
                    var runToMatch = currentRow[runIndex]
                    if (canMerge(true, currentFace, runToMatch)) {
                        faceBuffer.put(runToMatch[3], faceBuffer[runToMatch[3]] + 0x00100000)
                        runToMatch[2] = z
                        area++
                        z++
                        continue
                    }
                    runToMatch = lastRow[z]
                    if (canMerge(false, currentFace, runToMatch)) {
                        val startZ = z
                        val runEnd = runToMatch[2]
                        while (++z <= runEnd) {
                            val nextI = x shl 8 or (y shl 4) or z
                            if (blocks[nextI].model == null) {
                                break
                            }
                            if (isFaceCulled(Face.UP, x, y, z, nextI)) {
                                break
                            }
                            val nextFace = blocks[nextI].model.faces[Face.UP.ordinal]
                            if (!canMerge(false, currentFace, runToMatch)) {
                                break
                            }
                        }
                        runIndex = z--
                        currentRow[startZ][0] = runToMatch[0]
                        currentRow[startZ][1] = runToMatch[1]
                        currentRow[startZ][2] = z
                        if (z == runEnd) {
                            faceBuffer.put(runToMatch[3], faceBuffer[runToMatch[3]] + 0x10000000)
                            currentRow[startZ][3] = runToMatch[3]
                        } else {
                            faceBuffer.put(currentFace[0] + xOffset + yOffset + zOffset)
                            faceBuffer.put(currentFace[1] + 0x00100000 * (z - startZ))
                            currentRow[startZ][3] = faceBuffer.position() - 1
                        }
                        area += z - startZ
                    } else {
                        faceBuffer.put(currentFace[0] + xOffset + yOffset + zOffset)
                        faceBuffer.put(currentFace[1])
                        runIndex = z
                        currentRow[z][0] = currentFace[0]
                        currentRow[z][1] = currentFace[1]
                        currentRow[z][2] = z
                        currentRow[z][3] = faceBuffer.position() - 1
                        area++
                    }
                    z++
                }
                val temp = lastRow
                lastRow = currentRow
                currentRow = temp
                runIndex = 0
                clearRow(currentRow)
            }
            clearRow(lastRow)
        }
        counts[Face.UP.ordinal] = (faceBuffer.position() - startPos) * 5 / 2
    }

    enum class Face {
        SOUTH, NORTH, WEST, EAST, DOWN, UP
    }

    companion object {
        /**
         * Checks if the current face can merge with the active run in the same row
         * Requires minX of the current face to be 0
         * Requires maxX of the current run to be 15
         * Requires minY of the current face to equal minY of the current face
         * Requires minY of the current face to equal minY of the current face
         */
        private fun canDoWidthMerge(runToMatch: QuadRun, currentFace: IntArray): Boolean {
            return if (!mergeWidth) {
                false
            } else {
                runToMatch.texture == BlockModel.texture(currentFace) &&
                        runToMatch.startX + runToMatch.startWidth == 15 &&
                        BlockModel.x(currentFace) == 0 &&
                        runToMatch.startY == BlockModel.y(currentFace) &&
                        runToMatch.startHeight == BlockModel.height(currentFace)
            }
        }

        /**
         * Checks if the current face can begin merging with the run in the last row
         * Requires minX of the current face and run to be equal
         * Requires minY of the current face to be 0
         * Requires maxY of the run to be 15
         */
        private fun canStartHeightMerge(runToMatch: QuadRun, currentFace: IntArray?): Boolean {
            return if (!mergeHeight) {
                false
            } else {
                runToMatch.texture == BlockModel.texture(currentFace) &&
                        BlockModel.y(currentFace) == 0 &&
                        runToMatch.endY + runToMatch.endHeight == 15 &&
                        BlockModel.x(currentFace) == runToMatch.startX
            }
        }

        /**
         * Checks if the current face can merge with the next face
         * Requires maxX of the current face to be 15
         * Requires minX of the next face to be 0
         * Requires minY of the next face to be 0
         * Requires maxY of the current face to be equal maxY of the next face
         */
        private fun canContinueHeightMerge(currentFace: IntArray?, nextFace: IntArray): Boolean {
            return BlockModel.texture(currentFace) == BlockModel.texture(nextFace) && BlockModel.x(currentFace) + BlockModel.width(currentFace) == 15 && BlockModel.x(nextFace) == 0 && BlockModel.y(currentFace) == BlockModel.y(nextFace) && BlockModel.height(currentFace) == BlockModel.height(nextFace)
        }

        /**
         * Checks if the current face can finish merging with the run in the last row
         * Requires maxX of the current face and run to be equal
         */
        private fun canFinishHeightMerge(runToMatch: QuadRun, currentFace: IntArray?): Boolean {
            return runToMatch.texture == BlockModel.texture(currentFace) &&
                    BlockModel.y(currentFace) == 0 &&
                    runToMatch.endY + runToMatch.endHeight == 15 &&
                    BlockModel.width(currentFace) == runToMatch.endWidth
        }

        //    private static boolean mergeHeight = false;
        //    private static boolean mergeWidth = false;
        private const val mergeHeight = true
        private const val mergeWidth = true
        private fun canMerge(sameRow: Boolean, runToMatch: IntArray, currentFace: IntArray): Boolean {
            return false
        }

        private fun clearRow(currentRow: Array<IntArray>) {
            for (j in 0..15) {
                currentRow[j][0] = 0
                currentRow[j][1] = 0
                currentRow[j][2] = 0
                currentRow[j][3] = 0
                currentRow[j][4] = 0
                currentRow[j][5] = 0
            }
        }

        private fun clearRow(currentRow: Array<QuadRun>) {
            for (j in 0..15) {
                currentRow[j].reset()
            }
        }

        fun toVec(i: Int): Vector3i {
            return Vector3i(i and (0xf00 shr 8), i and (0x0f0 shr 4), i and 0x00f)
        }

        fun toVec(i: Int, vec: Vector3i): Vector3i {
            return vec.set(i and 0xf00 shr 8, i and 0x0f0 shr 4, i and 0x00f)
        }

        fun toIndex(x: Int, y: Int, z: Int): Int {
            return x shl 8 or (y shl 4) or z
        }
    }
}