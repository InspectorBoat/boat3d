package boat.world;

import boat.block.BlockRegistry;
import boat.block.BlockState;
import org.joml.Vector3i;
import org.lwjgl.PointerBuffer;
import org.lwjgl.system.MemoryStack;
import org.lwjgl.system.MemoryUtil;

import java.nio.IntBuffer;
import java.util.ArrayList;

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
//                    if (j % 2 == 0) blocks[i] = BlockRegistry.registry[1];
//                    else blocks[i] = BlockRegistry.registry[0];

                    double noiseVal = this.world.noise.eval(x / 16d + this.chunkPos.x, y / 16d + this.chunkPos.y, z / 16d + this.chunkPos.z);
                    blocks[i] = BlockRegistry.registry[noiseVal < 0.1 ? 0 : 1];

//                    blocks[i] = BlockRegistry.registry[1];

//                    if (z == 15 || x == 15 || y == 15) blocks[i] = BlockRegistry.registry[1];
//                    else if (z == 0 || x == 0 || y == 0) blocks[i] = BlockRegistry.registry[1];
//                    else blocks[i] = BlockRegistry.registry[0];

                    i++;
                    j++;
                }
                j++;
            }
            j++;
        }
//        System.out.printf("Generated chunk in %d ms\n", System.currentTimeMillis() - start);
    }

    @SuppressWarnings("ConstantConditions")
    public void buildMesh(MemoryStack stack) {
        if (this.buffer == -1) this.buffer = glCreateBuffers();

        // [texture0, texture1, width, faces index];
        //Reuse these arrays to prevent allocating an obscene amount of memory
        int[][] nextRow = new int[16][4];
        int[][] thisRow = new int[16][4];
        String uploadMode = "buffer";
        if (uploadMode.equals("buffer")) {
            try (MemoryStack st = stack.push()) {
                IntBuffer faceBuffer = st.mallocInt(16384);

                this.meshSouth(faceBuffer, thisRow, nextRow);
                this.meshNorth(faceBuffer, thisRow, nextRow);
                this.meshWest(faceBuffer, thisRow, nextRow);
                this.meshEast(faceBuffer, thisRow, nextRow);
                this.meshDown(faceBuffer, thisRow, nextRow);
                this.meshUp(faceBuffer, thisRow, nextRow);

                this.faceCount = faceBuffer.position() / 2;
                faceBuffer.limit(faceBuffer.position());
                faceBuffer.position(0);

                glNamedBufferStorage(this.buffer, (faceBuffer.limit() + 4L) * Integer.BYTES, GL_DYNAMIC_STORAGE_BIT | GL_MAP_WRITE_BIT);
                glNamedBufferSubData(this.buffer, 0, new int[] { this.chunkPos.x, this.chunkPos.y, this.chunkPos.z });
                glNamedBufferSubData(this.buffer, 4 * Integer.BYTES, faceBuffer);
            }
        }
        else if (uploadMode.equals("list")) {
            ArrayList<Integer> faceList = new ArrayList<>();
            this.meshSouth2(faceList, nextRow, thisRow);
            this.faceCount = faceList.size() / 2;
            int[] mesh = new int[faceList.size() + 4];
            mesh[0] = this.chunkPos.x;
            mesh[1] = this.chunkPos.y;
            mesh[2] = this.chunkPos.z;
            mesh[3] = 0;
            for (int i = 0; i < faceList.size(); i++) {
                mesh[i + 4] = faceList.get(i);
            }
            glNamedBufferStorage(this.buffer, (long) mesh.length * Integer.BYTES, GL_DYNAMIC_STORAGE_BIT | GL_MAP_WRITE_BIT);
            glNamedBufferSubData(this.buffer, 0, mesh);
        }
        this.countBuffer.put(this.counts);
        this.offsetBuffer.put(this.offsets);
        this.countBuffer.position(0);
        this.offsetBuffer.position(0);
        this.doneMeshing = true;
    }

    public boolean isFaceCulled(Face face, int x, int y, int z, int i) {
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

    public void meshSouth(IntBuffer faceBuffer, int[][] currentRow, int[][] lastRow) {
        this.offsets[Face.SOUTH.ordinal()] = faceBuffer.position() * 5L / 2 * Integer.BYTES;
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

                    if (this.blocks[i].model == null || isFaceCulled(Face.SOUTH, x, y, z, i)) {
                        activeRun = false;
                        continue;
                    }

                    int[] currentFace = this.blocks[i].model.faces[Face.SOUTH.ordinal()];

                    //Check if face can be merged with run in current row
                    if (activeRun) {
                        int[] runToMatch = currentRow[runIndex];
                        if (currentFace[0] == runToMatch[0] && currentFace[1] == runToMatch[1]) {
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
                    if (currentFace[0] == runToMatch[0] && currentFace[1] == runToMatch[1]) {
                        int startY = y;
                        //Index where the previous run ended
                        int runEnd = runToMatch[2];
                        //If run length 1, this doesn't trigger at all. This should never roll over
                        while (++ y <= runEnd) {
                            i = (x << 8) | (y << 4) | z;

                            if (this.blocks[i].model == null) break;
                            if (isFaceCulled(Face.SOUTH, x, y, z, i)) break;
                            currentFace = this.blocks[i].model.faces[Face.SOUTH.ordinal()];
                            if (currentFace[0] != runToMatch[0] || currentFace[1] != runToMatch[1]) {
                                break;
                            }
                        }
                        //Reset run info
                        activeRun = false;
                        //Pull down info
                        currentRow[startY][0] = runToMatch[0];
                        currentRow[startY][1] = runToMatch[1];
                        //Decrement y again to account for overshooting
                        currentRow[startY][2] = -- y;
                        //Matched entire run
                        if (y == runEnd) {
                            //Extend the quad downwards
                            faceBuffer.put(runToMatch[3], faceBuffer.get(runToMatch[3]) + 0x10000000);

                            //Pull down info
                            currentRow[startY][3] = runToMatch[3];
                        }
                        //Didn't match entire run
                        else {
                            //Start a new quad
                            faceBuffer.put(currentFace[0] + xOffset + yOffset + zOffset);
                            faceBuffer.put(currentFace[1] + 0x00100000 * (y - startY));
                            currentRow[startY][3] = faceBuffer.position() - 1;
                        }

                        this.area += (y - startY);
                        continue;
                    }

                    //Otherwise, start a new quad
                    faceBuffer.put(currentFace[0] + xOffset + yOffset + zOffset);
                    faceBuffer.put(currentFace[1]);
                    //Add run info entry
                    runIndex = y;
                    activeRun = true;

                    currentRow[y][0] = currentFace[0];
                    currentRow[y][1] = currentFace[1];
                    currentRow[y][2] = y;
                    currentRow[y][3] = faceBuffer.position() - 1;
                    this.area ++;
                }
                int[][] temp = lastRow;
                lastRow = currentRow;
                currentRow = temp;
                activeRun = false;
                clearRow(currentRow);
            }
            clearRow(lastRow);
        }
        this.counts[Face.SOUTH.ordinal()] = (faceBuffer.position() - startPos) * 5 / 2;
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
                        if (currentFace[0] == runToMatch[0] && currentFace[1] == runToMatch[1]) {
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
                    if (currentFace[0] == runToMatch[0] && currentFace[1] == runToMatch[1]) {
                        int startY = y;
                        //Index where the previous run ended
                        int runEnd = runToMatch[2];
                        //If run length 1, this doesn't trigger at all. This should never roll over

                        while (++ y <= runEnd) {
                            i = (x << 8) | (y << 4) | z;

                            if (this.blocks[i].model == null) break;
                            if (isFaceCulled(Face.NORTH, x, y, z, i)) break;
                            int[] nextFace = this.blocks[i].model.faces[Face.NORTH.ordinal()];
                            if (nextFace[0] != runToMatch[0] || nextFace[1] != runToMatch[1]) {
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
                    if (currentFace[0] == runToMatch[0] && currentFace[1] == runToMatch[1]) {
                        faceBuffer.put(runToMatch[3], faceBuffer.get(runToMatch[3]) + 0x10000000);

                        runToMatch[2] = z;
                        this.area ++;
                        continue;
                    }

                    runToMatch = lastRow[z];
                    if (currentFace[0] == runToMatch[0] && currentFace[1] == runToMatch[1]) {
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
                            if (nextFace[0] != runToMatch[0] || nextFace[1] != runToMatch[1]) {
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
                    if (currentFace[0] == runToMatch[0] && currentFace[1] == runToMatch[1]) {
                        faceBuffer.put(runToMatch[3], faceBuffer.get(runToMatch[3]) + 0x10000000);

                        runToMatch[2] = z;
                        this.area ++;
                        continue;
                    }

                    runToMatch = lastRow[z];
                    if (currentFace[0] == runToMatch[0] && currentFace[1] == runToMatch[1]) {
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
                            if (nextFace[0] != runToMatch[0] || nextFace[1] != runToMatch[1]) {
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
                    if (currentFace[0] == runToMatch[0] && currentFace[1] == runToMatch[1]) {
                        faceBuffer.put(runToMatch[3], faceBuffer.get(runToMatch[3]) + 0x00100000);

                        runToMatch[2] = z;
                        this.area ++;
                        continue;
                    }

                    runToMatch = lastRow[z];
                    if (currentFace[0] == runToMatch[0] && currentFace[1] == runToMatch[1]) {
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
                            if (nextFace[0] != runToMatch[0] || nextFace[1] != runToMatch[1]) {
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
                    if (currentFace[0] == runToMatch[0] && currentFace[1] == runToMatch[1]) {
                        faceBuffer.put(runToMatch[3], faceBuffer.get(runToMatch[3]) + 0x00100000);

                        runToMatch[2] = z;
                        this.area ++;
                        continue;
                    }

                    runToMatch = lastRow[z];
                    if (currentFace[0] == runToMatch[0] && currentFace[1] == runToMatch[1]) {
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
                            if (nextFace[0] != runToMatch[0] || nextFace[1] != runToMatch[1]) {
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

    public void meshSouth2(ArrayList<Integer> faceList, int[][] currentRow, int[][] lastRow) {
        int runIndex = 0;

        this.offsets[Face.SOUTH.ordinal()] = (faceList.size()) * 5L / 2 * Integer.BYTES;
        for (int z = 0; z < 16; z++) {
            int zOffset = z * 16 << 8;

            for (int x = 0; x < 16; x++) {

                int xOffset = x * 16 << 24;

                for (int y = 0; y < 16; y++) {
                    int yOffset = y * 16 << 16;
                    int i = toIndex(x, y, z);

                    if (this.blocks[i].model == null || isFaceCulled(Face.SOUTH, x, y, z, i)) {
                        runIndex = y;
                        continue;
                    }

                    int[] currentFace = this.blocks[i].model.faces[Face.SOUTH.ordinal()];
                    int[] runToMatch = currentRow[runIndex];
                    //Check if face can be merged with run in current row
                    if (currentFace[0] == runToMatch[0] && currentFace[1] == runToMatch[1]) {
                        //Add onto the last quad
                        faceList.set(runToMatch[3], faceList.get(runToMatch[3]) + 0x00100000);

                        //Increment merge run length
                        runToMatch[2] = y;
                        this.area ++;
                        continue;
                    }

                    //Try to merge with run from previous row
                    runToMatch = lastRow[y];
                    if (currentFace[0] == runToMatch[0] && currentFace[1] == runToMatch[1]) {
                        int startY = y;
                        //Index where the previous run ended
                        int runEnd = runToMatch[2];
                        //If run length 1, this doesn't trigger at all. This should never roll over

                        while (++y <= runEnd) {
                            int nextI = (x << 8) | (y << 4) | z;

                            if (this.blocks[nextI].model == null) {
                                break;

                            }
                            if (isFaceCulled(Face.SOUTH, x, y, z, nextI)) {
                                break;

                            }
                            int[] nextFace = this.blocks[nextI].model.faces[Face.SOUTH.ordinal()];
                            if (nextFace[0] != runToMatch[0] || nextFace[1] != runToMatch[1]) {
                                break;
                            }
                        }
                        //Decrement y again to account for overshooting
                        //Reset run index
                        runIndex = y--;
                        //Pull down info
                        currentRow[startY][0] = runToMatch[0];
                        currentRow[startY][1] = runToMatch[1];
                        currentRow[startY][2] = y;
                        //Matched entire run
                        if (y == runEnd) {
                            //Extend the quad downwards
                            faceList.set(runToMatch[3], faceList.get(runToMatch[3]) + 0x10000000);

                            //Pull down info
                            currentRow[startY][3] = runToMatch[3];
                        }
                        //Didn't match entire run
                        else {
                            //Start a new quad
                            faceList.add(currentFace[0] + xOffset + yOffset + zOffset);
                            faceList.add(currentFace[1] + 0x00100000 * (y - startY));
                            currentRow[startY][3] = faceList.size() - 1;
                        }

                        this.area += (y - startY);
                    }
                    //Otherwise, start a new quad
                    else {
                        faceList.add(currentFace[0] + xOffset + yOffset + zOffset);
                        faceList.add(currentFace[1]);
                        //Add merge info entry
                        runIndex = y;
                        currentRow[y][0] = currentFace[0];
                        currentRow[y][1] = currentFace[1];
                        currentRow[y][2] = y;
                        currentRow[y][3] = faceList.size() - 1;

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
        this.counts[Face.SOUTH.ordinal()] = faceList.size() * 5 / 2;
    }

    private static void clearRow(int[][] currentRow) {
        for (int j = 0; j < 16; j ++) {
            currentRow[j][0] = 0;
            currentRow[j][1] = 0;
            currentRow[j][2] = 0;
            currentRow[j][3] = 0;
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
