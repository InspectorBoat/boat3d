package boat.world;

import boat.block.BlockRegistry;
import boat.block.BlockState;
import org.joml.Vector3i;
import org.lwjgl.PointerBuffer;
import org.lwjgl.system.MemoryStack;
import org.lwjgl.system.MemoryUtil;

import java.nio.IntBuffer;

import static boat.block.BlockModel.*;
import static boat.util.DebugHelper.*;
import static boat.util.GlHelper.dir;
import static org.lwjgl.opengl.GL46C.*;

@SuppressWarnings("AutoBoxing")
public class Chunk {
    public BlockState[] blocks = new BlockState[4096];
    public int buffer = glCreateBuffers();
    public int faceCount;
    public int[] counts = new int[6];
    public long[] offsets = new long[6];
    public IntBuffer countBuffer = MemoryUtil.memCallocInt(6);
    public PointerBuffer offsetBuffer = MemoryUtil.memCallocPointer(6);
    public World world;
    public boolean doneMeshing;

    public Vector3i chunkPos;
    private int area;

    public Chunk(World world, int x, int y, int z) {
        this.world = world;
        this.chunkPos = new Vector3i(x, y, z);
    }

    public void generateBlocks() {
        int i = 0, j = 0;
        for (int x = 0; x < 16; x ++) {
            for (int y = 0; y < 16; y++) {
                for (int z = 0; z < 16; z++) {
//                    if (j % 2 == 0) blocks[i] = BlockRegistry.registry[3];
//                    else blocks[i] = BlockRegistry.registry[1];

                    double noiseVal = this.world.noise.eval(x / 16d + this.chunkPos.x, y / 16d + this.chunkPos.y, z / 16d + this.chunkPos.z);
//                    blocks[i] = BlockRegistry.registry[noiseVal < 0.1 ? (noiseVal < 0.05 ? 0 : 4) : 1];
                    blocks[i] = BlockRegistry.registry[switch ((int) (noiseVal * 12 + 1)) {
                        case 1, 2 -> 5;
                        case 3, 4 -> 1;
                        case 5 -> 3;
                        case 6 -> 4;
                        default -> 0;
                    }];
//                    double noiseVal = this.world.noise.eval(x / 16d + this.chunkPos.x, y / 16d + this.chunkPos.y, z / 16d + this.chunkPos.z);
//                    blocks[i] = BlockRegistry.registry[noiseVal < 0.1 ? 0 : 1];

//                    blocks[i] = BlockRegistry.registry[1];

//                    if (z == 15 || x == 15 || y == 15) blocks[i] = BlockRegistry.registry[1];
//                    else if (z == 0 || x == 0 || y == 0) blocks[i] = BlockRegistry.registry[1];
//                    else blocks[i] = BlockRegistry.registry[0];

//                    if (z == 0 && (y % 3 == 0 || x % 3 == 0)) blocks[i] = BlockRegistry.registry[1];
//                    else blocks[i] = BlockRegistry.registry[0];

//                    blocks[i] = BlockRegistry.registry[(Math.random() > 0.5) ? Math.random() > 0.7 ? 0 : (Math.random() > 0.5 ? 3: 1) : 2];
//                    blocks[i] = BlockRegistry.registry[(Math.random() > 0.5) ? 0 : 1];

                    i++;
                    j++;
                }
                j++;
            }
            j++;
        }
//        log("Generated chunk in %d ms", System.currentTimeMillis() - start);
    }

    public void buildMesh(MemoryStack stack, boolean mock) {
        if (this.buffer == -1) this.buffer = glCreateBuffers();
        this.area = 0;
        this.faceCount = 0;

        //[model0, model1, start, end, model2, model3, buffer index];
        int[][] nextRow = new int[16][7];
        int[][] thisRow = new int[16][7];
        try (MemoryStack st = stack.push()) {
            IntBuffer faceBuffer = st.mallocInt(st.getPointer() / 4);

            this.meshSouth(faceBuffer, thisRow, nextRow);
//            this.meshNorth(faceBuffer, thisRow, nextRow);
//            this.meshWest(faceBuffer, thisRow, nextRow);
//            this.meshEast(faceBuffer, thisRow, nextRow);
//            this.meshDown(faceBuffer, thisRow, nextRow);
//            this.meshUp(faceBuffer, thisRow, nextRow);

            if (mock) return;

            this.faceCount = faceBuffer.position() / 2;

            faceBuffer.limit(faceBuffer.position());
            faceBuffer.position(0);

            this.world.totalArea += this.area;
            this.world.totalFaces += this.faceCount;
            this.world.byteCount += this.faceCount * 4;

            glNamedBufferStorage(this.buffer, (faceBuffer.limit() + 4L) * Integer.BYTES, GL_DYNAMIC_STORAGE_BIT | GL_MAP_WRITE_BIT);
            glNamedBufferSubData(this.buffer, 0, new int[] { this.chunkPos.x, this.chunkPos.y, this.chunkPos.z });
            glNamedBufferSubData(this.buffer, 4 * Integer.BYTES, faceBuffer);
        }
        this.countBuffer.put(this.counts);
        this.offsetBuffer.put(this.offsets);
        this.countBuffer.position(0);
        this.offsetBuffer.position(0);
        this.doneMeshing = true;
    }


    public void meshSouth(IntBuffer faceBuffer, int[][] currentRow, int[][] lastRow) {
        nolog();
        this.offsets[Face.SOUTH.ordinal()] = faceBuffer.position() * 5L / 2 * Integer.BYTES;
        final int startPos = faceBuffer.position();

        int[] activeRun = null;
        boolean isSameRow = false;
        int[] currentFace = null;
        int expectedArea = 0;
        for (int z = 0; z < 16; z ++) {
            for (int y = 0; y < 16; y ++) {
                for (int x = 0; x < 16; x ++) {
                    int i = toIndex(x, y, z);

                    //Immediately stop if block lacks a model or is culled
                    if (this.blocks[i].model == null || isFaceCulled(Face.SOUTH, x, y, z, i)) {
                        if (activeRun != null) log("\n%02d %02d %02d | end active run", x, y, z);
                        activeRun = null;
                        continue;
                    } else expectedArea ++;
                    log("\n%02d %02d %02d | ", x, y, z);
                    currentFace = this.blocks[i].model.faces[Face.SOUTH.ordinal()];

                    //Try to merge to active run in current row
                    if (activeRun != null && isSameRow) {
                        if (canDoWidthMerge(activeRun, currentFace)) {
                            this.extendRunWidth(faceBuffer, activeRun, currentFace);
                            log("extend %02d %02d %02d", activeRun[2], y, z);
                            continue;
                        }
                        activeRun = null;
                    }

                    //Try to start merge with run in last row
                    if (activeRun == null) {
                        if (canStartHeightMerge(lastRow[x], currentFace)) {
                            activeRun = lastRow[x];
                            isSameRow = false;
                            log("begin merge : at %02d %02d %02d : length %02d ", x, y - 1, z, activeRun[3] - activeRun[2] + 1);
                        }
                    }

                    if (activeRun != null) {
                        //Check if merge is still possible with next face
                        if (activeRun[3] != x) {
                            int next = toIndex(x + 1, y, z);
                            if (this.blocks[next].model == null || isFaceCulled(Face.SOUTH, x + 1, y, z, next)) {
                                log("abort merge: missing face");
                                startNewExtendedRun(faceBuffer, currentRow, activeRun, currentFace,activeRun[2], x, y, z);
                                activeRun = null;
                            }
                            else if (!canContinueHeightMerge(currentFace, this.blocks[next].model.faces[Face.SOUTH.ordinal()])) {
                                log("abort merge: mismatched face");
                                startNewExtendedRun(faceBuffer, currentRow, activeRun, currentFace,activeRun[2], x, y, z);
                                activeRun = null;
                            }
                            else {
                                log("continue merge", activeRun[2], y - 1, z);
                            }
                        }
                        //If we've reached the end of the run, try to complete merge
                        else {
                            if (canFinishHeightMerge(activeRun, currentFace)) {
                                log("finish merge: merge success");
                                extendRunHeight(faceBuffer, activeRun, currentFace, currentRow);
                                activeRun = null;
                            }
                            //If merge failed, start a new run
                            else {
                                log("finish merge: merge failed");
                                activeRun = startNewExtendedRun(faceBuffer, currentRow, activeRun, currentFace,activeRun[2], x, y, z);
                                isSameRow = true;
                            }
                        }
                        continue;
                    }

                    //Start a new run if no merges are possible
                    log("begin new run", x, y, z);
                    activeRun = this.startNewRun(faceBuffer, currentRow, currentFace, x, y, z);
                    isSameRow = true;
                }
                int[][] temp = lastRow;
                lastRow = currentRow;
                currentRow = temp;
                activeRun = null;
                clearRow(currentRow);
            }
            clearRow(lastRow);
        }
        logn("\nGenerated area %d | Expected area %d", this.area, expectedArea);
        assert(this.area == expectedArea);
        this.counts[Face.SOUTH.ordinal()] = (faceBuffer.position() - startPos) * 5 / 2;
    }

    /**
     * Extends the active run in the current row
     */
    private void extendRunWidth(IntBuffer faceBuffer, int[] runToExtend, int[] face) {
        faceBuffer.put(runToExtend[6], faceBuffer.get(runToExtend[6]) + ((width(face) + 1) << 24));

        runToExtend[3] ++;
        runToExtend[4] = face[0];
        runToExtend[5] = face[1];
        this.area ++;
    }

    /**
     * Extends the active run in the last row
     * Copies the run information into the current row
     */
    private void extendRunHeight(IntBuffer faceBuffer, int[] runToExtend, int[] face, int[][] currentRow) {
        faceBuffer.put(runToExtend[6], faceBuffer.get(runToExtend[6]) + ((height(face) + 1) << 16));

        int[] activeRun = currentRow[runToExtend[2]];

        activeRun[0] = runToExtend[0];
        activeRun[1] = runToExtend[1];

        activeRun[2] = runToExtend[2];
        activeRun[3] = runToExtend[3];

        activeRun[4] = face[0];
        activeRun[5] = face[1];

        activeRun[6] = runToExtend[6];

        this.area += runToExtend[3] - runToExtend[2] + 1;


    }

    /**
     * Buffers a new run from the provided face
     * Returns the new active run
     */
    private int[] startNewRun(IntBuffer faceBuffer, int[][] currentRow, int[] face, int x, int y, int z) {
        faceBuffer.put(face[0] + (x * 16 << 24) + (y * 16 << 16) + (z * 16 << 8));
        faceBuffer.put(face[1]);

        int[] activeRun = currentRow[x];

        activeRun[0] = face[0];
        activeRun[1] = face[1];

        activeRun[2] = x;
        activeRun[3] = x;

        activeRun[4] = face[0];
        activeRun[5] = face[1];
        activeRun[6] = faceBuffer.position() - 1;

        this.area ++;

        return activeRun;
    }

    /**
     * Buffers a new run ranging from minX/minY of the first face to the maxX/maxY of the end face
     * Returns the new active run
     */
    private int[] startNewExtendedRun(IntBuffer faceBuffer, int[][] currentRow, int[] startFace, int[] endFace, int startX, int endX, int y, int z) {
        faceBuffer.put(startFace[0] + (startX * 16 << 24) + (y * 16 << 16) + (z * 16 << 8));
        faceBuffer.put(faceTexture(width(startFace) + 1 + 16 * (endX - startX - 1) + width(endFace), height(endFace), texture(endFace)));

        int[] activeRun = currentRow[startX];

        activeRun[0] = startFace[0];
        activeRun[1] = startFace[1];

        activeRun[2] = startX;
        activeRun[3] = endX;

        activeRun[4] = endFace[0];
        activeRun[5] = endFace[1];

        activeRun[6] = faceBuffer.position() - 1;

        this.area += (endX - startX + 1);
        return activeRun;
    }

    /**
     * Checks if the current face can merge with the active run in the same row
     * Requires minX of the current face to be 0
     * Requires maxX of the current run to be 15
     * Requires minY of the current face to equal minY of the current face
     * Requires minY of the current face to equal minY of the current face
     */
    private static boolean canDoWidthMerge(int[] runToMatch, int[] currentFace) {
        if (!mergeWidth) return false;
        return texture(runToMatch) == texture(currentFace) &&
                (x(runToMatch[4]) + width(runToMatch[5])) == 15 &&
                x(currentFace) == 0 &&
                y(runToMatch) == y(currentFace) &&
                height(runToMatch) == height(currentFace);
    }

    /**
     * Checks if the current face can begin merging with the run in the last row
     * Requires minX of the current face and run to be equal
     * Requires minY of the current face to be 0
     * Requires maxY of the run to be 15
     */
    private static boolean canStartHeightMerge(int[] runToMatch, int[] currentFace) {
        if (!mergeHeight) return false;
        return texture(runToMatch) == texture(currentFace) &&
                y(currentFace) == 0 &&
                y(runToMatch[4]) + height(runToMatch[5]) == 15 &&
                x(currentFace) == x(runToMatch);
    }

    /**
     * Checks if the current face can merge with the next face
     * Requires maxX of the current face to be 15
     * Requires minX of the next face to be 0
     * Requires minY of the next face to be 0
     * Requires maxY of the current face to be equal maxY of the next face
     */
    private static boolean canContinueHeightMerge(int[] currentFace, int[] nextFace) {
        return texture(currentFace) == texture(nextFace) &&
                x(currentFace) + width(currentFace) == 15 &&
                x(nextFace) == 0 &&
                y(currentFace) == y(nextFace) &&
                height(currentFace) == height(nextFace);
    }

    /**
     * Checks if the current face can finish merging with the run in the last row
     * Requires maxX of the current face and run to be equal
     */
    private static boolean canFinishHeightMerge(int[] runToMatch, int[] currentFace) {
        return texture(runToMatch) == texture(currentFace) &&
                y(currentFace) == 0 &&
                y(runToMatch[4]) + height(runToMatch[5]) == 15 &&
                width(currentFace) == width(runToMatch[5]);
    }

//    private static boolean mergeHeight = false;
//    private static boolean mergeWidth = false;
    private static boolean mergeHeight = true;
    private static boolean mergeWidth = true;

    public boolean isFaceCulled(Face face, int x, int y, int z, int i) {
//        if (true) return false;
        int offset = dir(face, (byte) x, (byte) y, (byte) z);
        if (offset >= 0 && offset <= 4095) {
            short[] offsetCull = this.blocks[offset].cullProfile(face, true);
            short[] thisCull = this.blocks[i].cullProfile(face, false);
            return (thisCull[0] != -1 && offsetCull[0] != -1 &&
                    offsetCull[0] <= thisCull[0] &&
                    offsetCull[1] <= thisCull[1] &&
                    offsetCull[2] >= thisCull[2] &&
                    offsetCull[3] >= thisCull[3]);
        }
        int chunkOffset = dir(face, (byte) this.chunkPos.x, (byte) this.chunkPos.y, (byte) this.chunkPos.z);
        if (chunkOffset >= 0 && chunkOffset <= 4095 && this.world.chunks[chunkOffset] != null) {
            short[] offsetCull = this.world.chunks[chunkOffset].blocks[offset - 8192].cullProfile(face, true);
            short[] thisCull = this.blocks[i].cullProfile(face, false);
            return (thisCull[0] != -1 && offsetCull[0] != -1 &&
                    offsetCull[0] <= thisCull[0] &&
                    offsetCull[1] <= thisCull[1] &&
                    offsetCull[2] >= thisCull[2] &&
                    offsetCull[3] >= thisCull[3]);
        }

        return false;
    }

    private static boolean canMerge(boolean sameRow, int[] runToMatch, int[] currentFace) {
        return false;
    }

    public void meshNorth(IntBuffer faceBuffer, int[][] currentRow, int[][] lastRow) {
        this.offsets[Face.NORTH.ordinal()] = faceBuffer.position() * 5L / 2 * Integer.BYTES;
        int startPos = faceBuffer.position();
        int runIndex = 0;
        boolean activeRun = false;
        for (int z = 0; z < 16; z++) {
            int zOffset = z * 16 << 8;

            for (int x = 0; x < 16; x++) {
                int xOffset = x * 16 << 24;

                for (int y = 0; y < 16; y++) {
                    int yOffset = y * 16 << 16;
                    int i = toIndex(x, y, z);

                    if (this.blocks[i].model == null || isFaceCulled(Face.NORTH, x, y, z, i)) {
                        runIndex = y;
                        activeRun = false;
                        continue;
                    }

                    int[] currentFace = this.blocks[i].model.faces[Face.NORTH.ordinal()];
                    //Check if face can be merged with run in current row
                    if (activeRun) {
                        int[] runToMatch = currentRow[runIndex];
                        if (canMerge(true, currentFace, runToMatch)) {
                            //Add onto the last quad
                            faceBuffer.put(runToMatch[3], faceBuffer.get(runToMatch[3]) + 0x00100000);

                            //Increment merge run length
                            runToMatch[2] = y;
                            this.area++;
                            continue;
                        }
                    }

                    //Try to merge with run from previous row
                    int[] runToMatch = lastRow[y];
                    if (canMerge(false, currentFace, runToMatch)) {
                        int startY = y;
                        //Index where the previous run ended
                        int runEnd = runToMatch[2];
                        //If run length 1, this doesn't trigger at all. This should never roll over

                        while (++ y <= runEnd) {
                            i = (x << 8) | (y << 4) | z;

                            if (this.blocks[i].model == null) break;
                            if (isFaceCulled(Face.NORTH, x, y, z, i)) break;
                            int[] nextFace = this.blocks[i].model.faces[Face.NORTH.ordinal()];
                            if (!canMerge(false, currentFace, runToMatch)) {
                                break;
                            }
                        }
                        activeRun = false;

                        currentRow[startY][0] = runToMatch[0];
                        currentRow[startY][1] = runToMatch[1];
                        currentRow[startY][2] = -- y;
                        if (y == runEnd) {
                            faceBuffer.put(runToMatch[3], faceBuffer.get(runToMatch[3]) + 0x10000000);
                            currentRow[startY][3] = runToMatch[3];
                        }
                        else {
                            faceBuffer.put(currentFace[0] + xOffset + yOffset + zOffset);
                            faceBuffer.put(currentFace[1] + 0x00100000 * (y - startY));
                            currentRow[startY][3] = faceBuffer.position() - 1;
                        }

                        this.area += (y - startY);
                        continue;
                    }
                    faceBuffer.put(currentFace[0] + xOffset + yOffset + zOffset);
                    faceBuffer.put(currentFace[1]);
                    //Add merge info entry
                    runIndex = y;
                    activeRun = true;

                    currentRow[y][0] = currentFace[0];
                    currentRow[y][1] = currentFace[1];
                    currentRow[y][2] = y;
                    currentRow[y][3] = faceBuffer.position() - 1;
                    this.area++;
                }
                int[][] temp = lastRow;
                lastRow = currentRow;
                currentRow = temp;
                runIndex = 0;
                clearRow(currentRow);
            }
            clearRow(lastRow);
        }
        this.counts[Face.NORTH.ordinal()] = (faceBuffer.position() - startPos) * 5 / 2;
    }

    public void meshWest(IntBuffer faceBuffer, int[][] currentRow, int[][] lastRow) {
        this.offsets[Face.WEST.ordinal()] = faceBuffer.position() * 5L / 2 * Integer.BYTES;
        int startPos = faceBuffer.position();
        int runIndex = 0;
        for (int x = 0; x < 16; x++) {
            int xOffset = x * 16 << 24;

            for (int y = 0; y < 16; y++) {
                int yOffset = y * 16 << 16;

                for (int z = 0; z < 16; z++) {
                    int zOffset = z * 16 << 8;
                    int i = toIndex(x, y, z);

                    if (this.blocks[i].model == null || isFaceCulled(Face.WEST, x, y, z, i)) {
                        runIndex = z;
                        continue;
                    }

                    int[] currentFace = this.blocks[i].model.faces[Face.WEST.ordinal()];
                    int[] runToMatch = currentRow[runIndex];
                    if (canMerge(true, currentFace, runToMatch)) {
                        faceBuffer.put(runToMatch[3], faceBuffer.get(runToMatch[3]) + 0x10000000);

                        runToMatch[2] = z;
                        this.area ++;
                        continue;
                    }

                    runToMatch = lastRow[z];
                    if (canMerge(true, currentFace, runToMatch)) {
                        int startZ = z;
                        int runEnd = runToMatch[2];

                        while (++z <= runEnd) {
                            int nextI = (x << 8) | (y << 4) | z;

                            if (this.blocks[nextI].model == null) {
                                break;

                            }
                            if (isFaceCulled(Face.WEST, x, y, z, nextI)) {
                                break;
                            }
                            int[] nextFace = this.blocks[nextI].model.faces[Face.WEST.ordinal()];
                            if (!canMerge(false, currentFace, runToMatch)) {
                                break;
                            }
                        }
                        runIndex = z--;
                        currentRow[startZ][0] = runToMatch[0];
                        currentRow[startZ][1] = runToMatch[1];
                        currentRow[startZ][2] = z;
                        if (z == runEnd) {
                            faceBuffer.put(runToMatch[3], faceBuffer.get(runToMatch[3]) + 0x00100000);
                            currentRow[startZ][3] = runToMatch[3];
                        }
                        else {
                            faceBuffer.put(currentFace[0] + xOffset + yOffset + zOffset);
                            faceBuffer.put(currentFace[1] + 0x10000000 * (z - startZ));
                            currentRow[startZ][3] = faceBuffer.position() - 1;
                        }

                        this.area += (z - startZ);
                    }
                    else {
                        faceBuffer.put(currentFace[0] + xOffset + yOffset + zOffset);
                        faceBuffer.put(currentFace[1]);
                        runIndex = z;
                        currentRow[z][0] = currentFace[0];
                        currentRow[z][1] = currentFace[1];
                        currentRow[z][2] = z;
                        currentRow[z][3] = faceBuffer.position() - 1;
                        this.area++;
                    }
                }
                int[][] temp = lastRow;
                lastRow = currentRow;
                currentRow = temp;
                runIndex = 0;
                clearRow(currentRow);
            }
            clearRow(lastRow);
        }
        this.counts[Face.WEST.ordinal()] = (faceBuffer.position() - startPos) * 5 / 2;
    }

    public void meshEast(IntBuffer faceBuffer, int[][] currentRow, int[][] lastRow) {
        this.offsets[Face.EAST.ordinal()] = faceBuffer.position() * 5L / 2 * Integer.BYTES;
        int startPos = faceBuffer.position();
        int runIndex = 0;
        for (int x = 0; x < 16; x++) {
            int xOffset = x * 16 << 24;

            for (int y = 0; y < 16; y++) {
                int yOffset = y * 16 << 16;

                for (int z = 0; z < 16; z++) {
                    int zOffset = z * 16 << 8;
                    int i = toIndex(x, y, z);

                    if (this.blocks[i].model == null || isFaceCulled(Face.EAST, x, y, z, i)) {
                        runIndex = z;
                        continue;
                    }

                    int[] currentFace = this.blocks[i].model.faces[Face.EAST.ordinal()];
                    int[] runToMatch = currentRow[runIndex];
                    if (canMerge(true, currentFace, runToMatch)) {
                        faceBuffer.put(runToMatch[3], faceBuffer.get(runToMatch[3]) + 0x10000000);

                        runToMatch[2] = z;
                        this.area ++;
                        continue;
                    }

                    runToMatch = lastRow[z];
                    if (canMerge(false, currentFace, runToMatch)) {
                        int startZ = z;
                        int runEnd = runToMatch[2];

                        while (++z <= runEnd) {
                            int nextI = (x << 8) | (y << 4) | z;

                            if (this.blocks[nextI].model == null) {
                                break;

                            }
                            if (isFaceCulled(Face.EAST, x, y, z, nextI)) {
                                break;
                            }
                            int[] nextFace = this.blocks[nextI].model.faces[Face.EAST.ordinal()];
                            if (!canMerge(false, currentFace, runToMatch)) {
                                break;
                            }
                        }
                        runIndex = z--;
                        currentRow[startZ][0] = runToMatch[0];
                        currentRow[startZ][1] = runToMatch[1];
                        currentRow[startZ][2] = z;
                        if (z == runEnd) {
                            faceBuffer.put(runToMatch[3], faceBuffer.get(runToMatch[3]) + 0x00100000);
                            currentRow[startZ][3] = runToMatch[3];
                        }
                        else {
                            faceBuffer.put(currentFace[0] + xOffset + yOffset + zOffset);
                            faceBuffer.put(currentFace[1] + 0x10000000 * (z - startZ));
                            currentRow[startZ][3] = faceBuffer.position() - 1;
                        }

                        this.area += (z - startZ);
                    }
                    else {
                        faceBuffer.put(currentFace[0] + xOffset + yOffset + zOffset);
                        faceBuffer.put(currentFace[1]);
                        runIndex = z;
                        currentRow[z][0] = currentFace[0];
                        currentRow[z][1] = currentFace[1];
                        currentRow[z][2] = z;
                        currentRow[z][3] = faceBuffer.position() - 1;
                        this.area++;
                    }
                }
                int[][] temp = lastRow;
                lastRow = currentRow;
                currentRow = temp;
                runIndex = 0;
                clearRow(currentRow);
            }
            clearRow(lastRow);
        }
        this.counts[Face.EAST.ordinal()] = (faceBuffer.position() - startPos) * 5 / 2;
    }

    public void meshDown(IntBuffer faceBuffer, int[][] currentRow, int[][] lastRow) {
        this.offsets[Face.DOWN.ordinal()] = faceBuffer.position() * 5L / 2 * Integer.BYTES;
        int startPos = faceBuffer.position();
        int runIndex = 0;
        for (int y = 0; y < 16; y++) {
            int yOffset = y * 16 << 16;

            for (int x = 0; x < 16; x++) {
                int xOffset = x * 16 << 24;


                for (int z = 0; z < 16; z++) {
                    int zOffset = z * 16 << 8;
                    int i = toIndex(x, y, z);

                    if (this.blocks[i].model == null || isFaceCulled(Face.DOWN, x, y, z, i)) {
                        runIndex = z;
                        continue;
                    }

                    int[] currentFace = this.blocks[i].model.faces[Face.DOWN.ordinal()];
                    int[] runToMatch = currentRow[runIndex];
                    if (canMerge(true, currentFace, runToMatch)) {
                        faceBuffer.put(runToMatch[3], faceBuffer.get(runToMatch[3]) + 0x00100000);

                        runToMatch[2] = z;
                        this.area ++;
                        continue;
                    }

                    runToMatch = lastRow[z];
                    if (canMerge(false, currentFace, runToMatch)) {
                        int startZ = z;
                        int runEnd = runToMatch[2];

                        while (++z <= runEnd) {
                            int nextI = (x << 8) | (y << 4) | z;

                            if (this.blocks[nextI].model == null) {
                                break;

                            }
                            if (isFaceCulled(Face.DOWN, x, y, z, nextI)) {
                                break;
                            }
                            int[] nextFace = this.blocks[nextI].model.faces[Face.DOWN.ordinal()];
                            if (!canMerge(false, currentFace, runToMatch)) {
                                break;
                            }
                        }
                        runIndex = z--;
                        currentRow[startZ][0] = runToMatch[0];
                        currentRow[startZ][1] = runToMatch[1];
                        currentRow[startZ][2] = z;
                        if (z == runEnd) {
                            faceBuffer.put(runToMatch[3], faceBuffer.get(runToMatch[3]) + 0x10000000);
                            currentRow[startZ][3] = runToMatch[3];
                        }
                        else {
                            faceBuffer.put(currentFace[0] + xOffset + yOffset + zOffset);
                            faceBuffer.put(currentFace[1] + 0x00100000 * (z - startZ));
                            currentRow[startZ][3] = faceBuffer.position() - 1;
                        }

                        this.area += (z - startZ);
                    }
                    else {
                        faceBuffer.put(currentFace[0] + xOffset + yOffset + zOffset);
                        faceBuffer.put(currentFace[1]);
                        runIndex = z;
                        currentRow[z][0] = currentFace[0];
                        currentRow[z][1] = currentFace[1];
                        currentRow[z][2] = z;
                        currentRow[z][3] = faceBuffer.position() - 1;
                        this.area++;
                    }
                }
                int[][] temp = lastRow;
                lastRow = currentRow;
                currentRow = temp;
                runIndex = 0;
                clearRow(currentRow);
            }
            clearRow(lastRow);
        }
        this.counts[Face.DOWN.ordinal()] = (faceBuffer.position() - startPos) * 5 / 2;
    }

    public void meshUp(IntBuffer faceBuffer, int[][] currentRow, int[][] lastRow) {
        this.offsets[Face.UP.ordinal()] = faceBuffer.position() * 5L / 2 * Integer.BYTES;
        int startPos = faceBuffer.position();
        int runIndex = 0;
        for (int y = 0; y < 16; y++) {
            int yOffset = y * 16 << 16;

            for (int x = 0; x < 16; x++) {
                int xOffset = x * 16 << 24;


                for (int z = 0; z < 16; z++) {
                    int zOffset = z * 16 << 8;
                    int i = toIndex(x, y, z);

                    if (this.blocks[i].model == null || isFaceCulled(Face.UP, x, y, z, i)) {
                        runIndex = z;
                        continue;
                    }

                    int[] currentFace = this.blocks[i].model.faces[Face.UP.ordinal()];
                    int[] runToMatch = currentRow[runIndex];
                    if (canMerge(true, currentFace, runToMatch)) {
                        faceBuffer.put(runToMatch[3], faceBuffer.get(runToMatch[3]) + 0x00100000);

                        runToMatch[2] = z;
                        this.area ++;
                        continue;
                    }

                    runToMatch = lastRow[z];
                    if (canMerge(false, currentFace, runToMatch)) {
                        int startZ = z;
                        int runEnd = runToMatch[2];

                        while (++z <= runEnd) {
                            int nextI = (x << 8) | (y << 4) | z;

                            if (this.blocks[nextI].model == null) {
                                break;

                            }
                            if (isFaceCulled(Face.UP, x, y, z, nextI)) {
                                break;
                            }
                            int[] nextFace = this.blocks[nextI].model.faces[Face.UP.ordinal()];
                            if (!canMerge(false, currentFace, runToMatch)) {
                                break;
                            }
                        }
                        runIndex = z--;
                        currentRow[startZ][0] = runToMatch[0];
                        currentRow[startZ][1] = runToMatch[1];
                        currentRow[startZ][2] = z;
                        if (z == runEnd) {
                            faceBuffer.put(runToMatch[3], faceBuffer.get(runToMatch[3]) + 0x10000000);
                            currentRow[startZ][3] = runToMatch[3];
                        }
                        else {
                            faceBuffer.put(currentFace[0] + xOffset + yOffset + zOffset);
                            faceBuffer.put(currentFace[1] + 0x00100000 * (z - startZ));
                            currentRow[startZ][3] = faceBuffer.position() - 1;
                        }

                        this.area += (z - startZ);
                    }
                    else {
                        faceBuffer.put(currentFace[0] + xOffset + yOffset + zOffset);
                        faceBuffer.put(currentFace[1]);
                        runIndex = z;
                        currentRow[z][0] = currentFace[0];
                        currentRow[z][1] = currentFace[1];
                        currentRow[z][2] = z;
                        currentRow[z][3] = faceBuffer.position() - 1;
                        this.area++;
                    }
                }
                int[][] temp = lastRow;
                lastRow = currentRow;
                currentRow = temp;
                runIndex = 0;
                clearRow(currentRow);
            }
            clearRow(lastRow);
        }
        this.counts[Face.UP.ordinal()] = (faceBuffer.position() - startPos) * 5 / 2;
    }

    private static void clearRow(int[][] currentRow) {
        for (int j = 0; j < 16; j ++) {
            currentRow[j][0] = 0;
            currentRow[j][1] = 0;
            currentRow[j][2] = 0;
            currentRow[j][3] = 0;
            currentRow[j][4] = 0;
            currentRow[j][5] = 0;
        }
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
        SOUTH,
        NORTH,
        WEST,
        EAST,
        DOWN,
        UP,
    }
}
