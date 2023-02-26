package boat.world;

import boat.block.BlockRegistry;
import boat.block.BlockState;
import org.joml.Vector3i;
import org.lwjgl.PointerBuffer;
import org.lwjgl.opengl.GL;
import org.lwjgl.system.MemoryStack;
import org.lwjgl.system.MemoryUtil;

import java.nio.IntBuffer;
import java.util.ArrayList;
import java.util.Arrays;

import static org.lwjgl.opengl.GL46C.*;
import static boat.util.GlHelper.dir;
import static org.lwjgl.system.Checks.CHECKS;
import static org.lwjgl.system.Checks.check;
import static org.lwjgl.system.JNI.callPPPV;

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
//                    if (z == 0 && x < 8) blocks[i] = BlockRegistry.registry[1];
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

    // TODO: Fix inner face culling bug
    @SuppressWarnings("ConstantConditions")
    public void buildMesh() {
        if (this.buffer == -1) this.buffer = glCreateBuffers();


        // [texture0, texture1, width, faces index];
        //Reuse these arrays to prevent allocating an obscene amount of memory
        int[][] nextRow = new int[16][4];
        int[][] thisRow = new int[16][4];
        String uploadMode = "buffer";
        if (uploadMode.equals("buffer")) {
            try (MemoryStack stack = MemoryStack.stackPush()) {
                IntBuffer faceBuffer = stack.mallocInt(stack.getPointer() / 4);

                this.meshSouth(faceBuffer, thisRow, nextRow);
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
        int runIndex = 0;

        this.offsets[Face.SOUTH.ordinal()] = (faceBuffer.position()) * 5L / 2 * Integer.BYTES;
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
                        faceBuffer.put(runToMatch[3], faceBuffer.get(runToMatch[3]) + 0x00100000);

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
                    }
                    //Otherwise, start a new quad
                    else {
                        faceBuffer.put(currentFace[0] + xOffset + yOffset + zOffset);
                        faceBuffer.put(currentFace[1]);
                        //Add merge info entry
                        runIndex = y;
                        currentRow[y][0] = currentFace[0];
                        currentRow[y][1] = currentFace[1];
                        currentRow[y][2] = y;
                        currentRow[y][3] = faceBuffer.position() - 1;
                        this.area++;
                    }
                }
                int[][] temp = lastRow;
                lastRow = currentRow;
                currentRow = temp;
                runIndex = 0;
                for (int j = 0; j < 16; j ++) {
                    currentRow[j][0] = 0;
                    currentRow[j][1] = 0;
                    currentRow[j][2] = 0;
                    currentRow[j][3] = 0;
                }
            }
            for (int j = 0; j < 16; j ++) {
                lastRow[j][0] = 0;
                lastRow[j][1] = 0;
                lastRow[j][2] = 0;
                lastRow[j][3] = 0;
            }
        }
        this.counts[Face.SOUTH.ordinal()] = (faceBuffer.position()) * 5 / 2;
    }

    public void meshNorth(ArrayList<Integer> faceList, int[][] thisRow, int[][] lastRow) {
        int mergeIndex = 0;
        this.offsets[Face.NORTH.ordinal()] = faceList.size() * 5 / 2 * Integer.BYTES;
        for (int z = 0; z < 16; z++) {
            int zOffset = z * 16 << 8;

            for (int x = 0; x < 16; x++) {
                int xOffset = x * 16 << 24;

                for (int y = 0; y < 16; y++) {
                    int yOffset = y * 16 << 16;
                    int i = toIndex(x, y, z);

                    if (this.blocks[i].model == null || isFaceCulled(Face.NORTH, x, y, z, i)) {
                        mergeIndex = y;
                        continue;
                    }

                    int[] face = this.blocks[i].model.faces[Face.NORTH.ordinal()];

                    //Check if face can be merged with run in current row
                    if (face[0] == thisRow[mergeIndex][0] && face[1] == thisRow[mergeIndex][1]) {
                        //Add onto the last quad
                        faceList.set(thisRow[mergeIndex][3], faceList.get(thisRow[mergeIndex][3]) + 0x00100000);
                        //Increment merge run length
                        thisRow[mergeIndex][2] ++;
                        area ++;
                        continue;
                    }

                    //Check it might be possible to merge with run from previous row
                    int[] previousRowRun = lastRow[y];
                    if (face[0] == previousRowRun[0] && face[1] == previousRowRun[1]) {
                        int startY = y;
                        //How long our run is, since we've already matched one, this starts at 1
                        int completed = 1;
                        //If run length 1, this doesn't trigger at all. This should never roll over
                        while (completed < previousRowRun[2]) {
                            int nextI = (x << 8) | ((y + 1) << 4) | z;

                            if (this.blocks[nextI].model == null) {
                                break;
                            }
                            if (isFaceCulled(Face.NORTH, x, y + 1, z, nextI)) {
                                break;
                            }
                            int[] nextFace = this.blocks[nextI].model.faces[Face.NORTH.ordinal()];
                            if (nextFace[0] != previousRowRun[0] || nextFace[1] != previousRowRun[1]) {
                                break;
                            }

                            y ++;
                            completed ++;
                            thisRow[y][0] = 0;
                            thisRow[y][1] = 0;
                            thisRow[y][2] = 0;
                            thisRow[y][3] = 0;
                        }
                        //We managed to match the entire run
                        if (completed == previousRowRun[2]) {
                            //Extend the quad downwards
                            faceList.set(previousRowRun[3], faceList.get(previousRowRun[3]) + 0x10000000);

                            //Pull down info
                            thisRow[startY][0] = previousRowRun[0];
                            thisRow[startY][1] = previousRowRun[1];
                            thisRow[startY][2] = completed;
                            thisRow[startY][3] = previousRowRun[3];
                            //Reset merge index
                            mergeIndex = y + 1;
                        }
                        //Didn't match entire run
                        else {
                            //Start a new quad
                            faceList.add(face[0] + xOffset + yOffset + zOffset);
                            faceList.add(face[1] + 0x00100000 * (completed - 1));

                            //Reset merge index
                            mergeIndex = y;
                            //Pull down info
                            thisRow[startY][0] = previousRowRun[0];
                            thisRow[startY][1] = previousRowRun[1];
                            thisRow[startY][2] = completed;
                            thisRow[startY][3] = faceList.size() - 1;
                        }
                        area += completed;
                        continue;
                    }

                    //Otherwise, start a new quad
                    faceList.add(face[0] + xOffset + yOffset + zOffset);
                    faceList.add(face[1]);
                    //Add merge info entry
                    mergeIndex = y;
                    thisRow[y][0] = face[0];
                    thisRow[y][1] = face[1];
                    thisRow[y][2] = 1;
                    thisRow[y][3] = faceList.size() - 1;
                    area ++;
                }
                int[][] temp = lastRow;
                lastRow = thisRow;
                thisRow = temp;
                mergeIndex = 0;
                for (int j = 0; j < 16; j ++) {
                    thisRow[j][0] = 0;
                    thisRow[j][1] = 0;
                    thisRow[j][2] = 0;
                    thisRow[j][3] = 0;
                }
            }
            for (int j = 0; j < 16; j ++) {
                lastRow[j][0] = 0;
                lastRow[j][1] = 0;
                lastRow[j][2] = 0;
                lastRow[j][3] = 0;
            }
        }
        this.counts[Face.NORTH.ordinal()] = faceList.size() * 5 / 2;

    }

    public void meshWest(ArrayList<Integer> faceList, int[][] thisRow, int[][] lastRow) {
        int mergeIndex = 0;
        this.offsets[Face.WEST.ordinal()] = faceList.size() * 5 / 2 * Integer.BYTES;
        for (int x = 0; x < 16; x++) {
            int xOffset = x * 16 << 24;

            for (int y = 0; y < 16; y++) {
                int yOffset = y * 16 << 16;

                for (int z = 0; z < 16; z++) {
                    int zOffset = z * 16 << 8;

                    int i = toIndex(x, y, z);
                    if (this.blocks[i].model == null || isFaceCulled(Face.WEST, x, y, z, i)) {
                        mergeIndex = z;
                        continue;
                    }

                    int[] face = this.blocks[i].model.faces[Face.WEST.ordinal()];

                    //Check if face can be merged with run in current row
                    if (face[0] == thisRow[mergeIndex][0] && face[1] == thisRow[mergeIndex][1]) {
                        //Add onto the last quad
                        faceList.set(thisRow[mergeIndex][3], faceList.get(thisRow[mergeIndex][3]) + 0x10000000);
                        //Increment merge run length
                        thisRow[mergeIndex][2] ++;
                        area ++;
                        continue;
                    }

                    //Check it might be possible to merge with run from previous row
                    int[] previousRowRun = lastRow[z];
                    if (face[0] == previousRowRun[0] && face[1] == previousRowRun[1]) {
                        int startZ = z;
                        //How long our run is, since we've already matched one, this starts at 1
                        int completed = 1;
                        //If run length 1, this doesn't trigger at all. This should never roll over
                        while (completed < previousRowRun[2]) {
                            int nextI = (x << 8) | (y << 4) | (z + 1);

                            if (this.blocks[nextI].model == null) {
                                break;
                            }
                            if (isFaceCulled(Face.WEST, x, y, z + 1, nextI)) {
                                break;
                            }
                            int[] nextFace = this.blocks[nextI].model.faces[Face.WEST.ordinal()];
                            if (nextFace[0] != previousRowRun[0] || nextFace[1] != previousRowRun[1]) {
                                break;
                            }

                            z ++;
                            completed ++;
                        }
                        //We managed to match the entire run
                        if (completed == previousRowRun[2]) {
                            //Extend the quad downwards
                            faceList.set(previousRowRun[3], faceList.get(previousRowRun[3]) + 0x00100000);

                            //Pull down info
                            thisRow[startZ][0] = previousRowRun[0];
                            thisRow[startZ][1] = previousRowRun[1];
                            thisRow[startZ][2] = previousRowRun[2];
                            thisRow[startZ][3] = previousRowRun[3];
                            //Reset merge index
                            mergeIndex = z + 1;

                        }
                        //Didn't match entire run
                        else {
                            //Start a new quad
                            faceList.add(face[0] + xOffset + yOffset + zOffset);
                            faceList.add(face[1] + 0x10000000 * (completed - 1));

                            //Reset merge index
                            mergeIndex = z;
                            //Pull down info
                            thisRow[startZ][0] = previousRowRun[0];
                            thisRow[startZ][1] = previousRowRun[1];
                            thisRow[startZ][2] = completed;
                            thisRow[startZ][3] = faceList.size() - 1;
                        }
                        area += completed;
                        continue;
                    }

                    //Otherwise, start a new quad
                    faceList.add(face[0] + xOffset + yOffset + zOffset);
                    faceList.add(face[1]);
                    //Add merge info entry
                    mergeIndex = z;

                    thisRow[z][0] = face[0];
                    thisRow[z][1] = face[1];
                    thisRow[z][2] = 1;
                    thisRow[z][3] = faceList.size() - 1;
                    area ++;
                }
                int[][] temp = lastRow;
                lastRow = thisRow;
                thisRow = temp;
                mergeIndex = 0;
                for (int j = 0; j < 16; j ++) {
                    thisRow[j][0] = 0;
                    thisRow[j][1] = 0;
                    thisRow[j][2] = 0;
                    thisRow[j][3] = 0;
                }
            }
            for (int j = 0; j < 16; j ++) {
                lastRow[j][0] = 0;
                lastRow[j][1] = 0;
                lastRow[j][2] = 0;
                lastRow[j][3] = 0;
            }
        }
        this.counts[Face.WEST.ordinal()] = faceList.size() * 5 / 2;

    }

    public void meshEast(ArrayList<Integer> faceList, int[][] thisRow, int[][] lastRow) {
        int mergeIndex = 0;
        this.offsets[Face.EAST.ordinal()] = faceList.size() * 5 / 2 * Integer.BYTES;
        for (int x = 0; x < 16; x++) {
            int xOffset = x * 16 << 24;

            for (int y = 0; y < 16; y++) {
                int yOffset = y * 16 << 16;

                for (int z = 0; z < 16; z++) {
                    int zOffset = z * 16 << 8;

                    int i = toIndex(x, y, z);
                    if (this.blocks[i].model == null || isFaceCulled(Face.EAST, x, y, z, i)) {
                        mergeIndex = z;
                        continue;
                    }

                    int[] face = this.blocks[i].model.faces[Face.EAST.ordinal()];

                    //Check if face can be merged with run in current row
                    if (face[0] == thisRow[mergeIndex][0] && face[1] == thisRow[mergeIndex][1]) {
                        //Add onto the last quad
                        faceList.set(thisRow[mergeIndex][3], faceList.get(thisRow[mergeIndex][3]) + 0x10000000);
                        //Increment merge run length
                        thisRow[mergeIndex][2] ++;
                        area ++;
                        continue;
                    }

                    //Check it might be possible to merge with run from previous row
                    int[] previousRowRun = lastRow[z];
                    if (face[0] == previousRowRun[0] && face[1] == previousRowRun[1]) {
                        int startZ = z;
                        //How long our run is, since we've already matched one, this starts at 1
                        int completed = 1;
                        //If run length 1, this doesn't trigger at all. This should never roll over
                        while (completed < previousRowRun[2]) {
                            int nextI = (x << 8) | (y << 4) | (z + 1);

                            if (this.blocks[nextI].model == null) {
                                break;
                            }
                            if (isFaceCulled(Face.EAST, x, y, z + 1, nextI)) {
                                break;
                            }
                            int[] nextFace = this.blocks[nextI].model.faces[Face.EAST.ordinal()];
                            if (nextFace[0] != previousRowRun[0] || nextFace[1] != previousRowRun[1]) {
                                break;
                            }

                            z ++;
                            completed ++;
                        }
                        //We managed to match the entire run
                        if (completed == previousRowRun[2]) {
                            //Extend the quad downwards
                            faceList.set(previousRowRun[3], faceList.get(previousRowRun[3]) + 0x00100000);

                            //Pull down info
                            thisRow[startZ][0] = previousRowRun[0];
                            thisRow[startZ][1] = previousRowRun[1];
                            thisRow[startZ][2] = previousRowRun[2];
                            thisRow[startZ][3] = previousRowRun[3];
                            //Reset merge index
                            mergeIndex = z + 1;

                        }
                        //Didn't match entire run
                        else {
                            //Start a new quad
                            faceList.add(face[0] + xOffset + yOffset + zOffset);
                            faceList.add(face[1] + 0x10000000 * (completed - 1));

                            //Reset merge index
                            mergeIndex = z;
                            //Pull down info
                            thisRow[startZ][0] = previousRowRun[0];
                            thisRow[startZ][1] = previousRowRun[1];
                            thisRow[startZ][2] = completed;
                            thisRow[startZ][3] = faceList.size() - 1;
                        }
                        area += completed;
                        continue;
                    }

                    //Otherwise, start a new quad
                    faceList.add(face[0] + xOffset + yOffset + zOffset);
                    faceList.add(face[1]);
                    //Add merge info entry
                    mergeIndex = z;

                    thisRow[z][0] = face[0];
                    thisRow[z][1] = face[1];
                    thisRow[z][2] = 1;
                    thisRow[z][3] = faceList.size() - 1;
                    area ++;
                }
                int[][] temp = lastRow;
                lastRow = thisRow;
                thisRow = temp;
                mergeIndex = 0;
                for (int j = 0; j < 16; j ++) {
                    thisRow[j][0] = 0;
                    thisRow[j][1] = 0;
                    thisRow[j][2] = 0;
                    thisRow[j][3] = 0;
                }
            }
            for (int j = 0; j < 16; j ++) {
                lastRow[j][0] = 0;
                lastRow[j][1] = 0;
                lastRow[j][2] = 0;
                lastRow[j][3] = 0;
            }
        }
        this.counts[Face.EAST.ordinal()] = faceList.size() * 5 / 2;
    }

    public void meshDown(ArrayList<Integer> faceList, int[][] thisRow, int[][] lastRow) {
        int mergeIndex = 0;
        this.offsets[Face.DOWN.ordinal()] = faceList.size() * 5 / 2 * Integer.BYTES;
        for (int y = 0; y < 16; y++) {
            int yOffset = y * 16 << 16;

            for (int x = 0; x < 16; x++) {
                int xOffset = x * 16 << 24;

                for (int z = 0; z < 16; z++) {
                    int zOffset = z * 16 << 8;

                    int i = toIndex(x, y, z);
                    if (this.blocks[i].model == null || isFaceCulled(Face.DOWN, x, y, z, i)) {
                        mergeIndex = z;
                        continue;
                    }

                    int[] face = this.blocks[i].model.faces[Face.DOWN.ordinal()];

                    //Check if face can be merged with run in current row
                    if (face[0] == thisRow[mergeIndex][0] && face[1] == thisRow[mergeIndex][1]) {
                        //Add onto the last quad
                        faceList.set(thisRow[mergeIndex][3], faceList.get(thisRow[mergeIndex][3]) + 0x00100000);
                        //Increment merge run length
                        thisRow[mergeIndex][2] ++;
                        area ++;
                        continue;
                    }

                    //Check it might be possible to merge with run from previous row
                    int[] previousRowRun = lastRow[z];
                    if (face[0] == previousRowRun[0] && face[1] == previousRowRun[1]) {
                        int startZ = z;
                        //How long our run is, since we've already matched one, this starts at 1
                        int completed = 1;
                        //If run length 1, this doesn't trigger at all. This should never roll over
                        while (completed < previousRowRun[2]) {
                            int nextI = (x << 8) | (y << 4) | (z + 1);

                            if (this.blocks[nextI].model == null) {
                                break;
                            }
                            if (isFaceCulled(Face.DOWN, x, y, z + 1, nextI)) {
                                break;
                            }
                            int[] nextFace = this.blocks[nextI].model.faces[Face.DOWN.ordinal()];
                            if (nextFace[0] != previousRowRun[0] || nextFace[1] != previousRowRun[1]) {
                                break;
                            }

                            z ++;
                            completed ++;
                        }
                        //We managed to match the entire run
                        if (completed == previousRowRun[2]) {
                            //Extend the quad downwards
                            faceList.set(previousRowRun[3], faceList.get(previousRowRun[3]) + 0x10000000);

                            //Pull down info
                            thisRow[startZ][0] = previousRowRun[0];
                            thisRow[startZ][1] = previousRowRun[1];
                            thisRow[startZ][2] = previousRowRun[2];
                            thisRow[startZ][3] = previousRowRun[3];
                            //Reset merge index
                            mergeIndex = z + 1;

                        }
                        //Didn't match entire run
                        else {
                            //Start a new quad
                            faceList.add(face[0] + xOffset + yOffset + zOffset);
                            faceList.add(face[1] + 0x00100000 * (completed - 1));

                            //Reset merge index
                            mergeIndex = z;
                            //Pull down info
                            thisRow[startZ][0] = previousRowRun[0];
                            thisRow[startZ][1] = previousRowRun[1];
                            thisRow[startZ][2] = completed;
                            thisRow[startZ][3] = faceList.size() - 1;
                        }
                        area += completed;
                        continue;
                    }

                    //Otherwise, start a new quad
                    faceList.add(face[0] + xOffset + yOffset + zOffset);
                    faceList.add(face[1]);
                    //Add merge info entry
                    mergeIndex = z;

                    thisRow[z][0] = face[0];
                    thisRow[z][1] = face[1];
                    thisRow[z][2] = 1;
                    thisRow[z][3] = faceList.size() - 1;
                    area ++;
                }
                int[][] temp = lastRow;
                lastRow = thisRow;
                thisRow = temp;
                mergeIndex = 0;
                for (int j = 0; j < 16; j ++) {
                    thisRow[j][0] = 0;
                    thisRow[j][1] = 0;
                    thisRow[j][2] = 0;
                    thisRow[j][3] = 0;
                }
            }
            for (int j = 0; j < 16; j ++) {
                lastRow[j][0] = 0;
                lastRow[j][1] = 0;
                lastRow[j][2] = 0;
                lastRow[j][3] = 0;
            }
        }
        this.counts[Face.DOWN.ordinal()] = faceList.size() * 5 / 2;

    }

    public void meshUp(ArrayList<Integer> faceList, int[][] thisRow, int[][] lastRow) {
        int mergeIndex = 0;
        this.offsets[Face.UP.ordinal()] = faceList.size() * 5 / 2 * Integer.BYTES;
        for (int y = 0; y < 16; y++) {
            int yOffset = y * 16 << 16;

            for (int x = 0; x < 16; x++) {
                int xOffset = x * 16 << 24;

                for (int z = 0; z < 16; z++) {
                    int zOffset = z * 16 << 8;

                    int i = toIndex(x, y, z);
                    if (this.blocks[i].model == null || isFaceCulled(Face.UP, x, y, z, i)) {
                        mergeIndex = z;
                        continue;
                    }

                    int[] face = this.blocks[i].model.faces[Face.UP.ordinal()];

                    //Check if face can be merged with run in current row
                    if (face[0] == thisRow[mergeIndex][0] && face[1] == thisRow[mergeIndex][1]) {
                        //Add onto the last quad
                        faceList.set(thisRow[mergeIndex][3], faceList.get(thisRow[mergeIndex][3]) + 0x00100000);
                        //Increment merge run length
                        thisRow[mergeIndex][2] ++;
                        area ++;
                        continue;
                    }

                    //Check it might be possible to merge with run from previous row
                    int[] previousRowRun = lastRow[z];
                    if (face[0] == previousRowRun[0] && face[1] == previousRowRun[1]) {
                        int startZ = z;
                        //How long our run is, since we've already matched one, this starts at 1
                        int completed = 1;
                        //If run length 1, this doesn't trigger at all. This should never roll over
                        while (completed < previousRowRun[2]) {
                            int nextI = (x << 8) | (y << 4) | (z + 1);

                            if (this.blocks[nextI].model == null) {
                                break;
                            }
                            if (isFaceCulled(Face.UP, x, y, z + 1, nextI)) {
                                break;
                            }
                            int[] nextFace = this.blocks[nextI].model.faces[Face.UP.ordinal()];
                            if (nextFace[0] != previousRowRun[0] || nextFace[1] != previousRowRun[1]) {
                                break;
                            }

                            z ++;
                            completed ++;
                        }
                        //We managed to match the entire run
                        if (completed == previousRowRun[2]) {
                            //Extend the quad downwards
                            faceList.set(previousRowRun[3], faceList.get(previousRowRun[3]) + 0x10000000);

                            //Pull down info
                            thisRow[startZ][0] = previousRowRun[0];
                            thisRow[startZ][1] = previousRowRun[1];
                            thisRow[startZ][2] = previousRowRun[2];
                            thisRow[startZ][3] = previousRowRun[3];
                            //Reset merge index
                            mergeIndex = z + 1;

                        }
                        //Didn't match entire run
                        else {
                            //Start a new quad
                            faceList.add(face[0] + xOffset + yOffset + zOffset);
                            faceList.add(face[1] + 0x00100000 * (completed - 1));

                            //Reset merge index
                            mergeIndex = z;
                            //Pull down info
                            thisRow[startZ][0] = previousRowRun[0];
                            thisRow[startZ][1] = previousRowRun[1];
                            thisRow[startZ][2] = completed;
                            thisRow[startZ][3] = faceList.size() - 1;
                        }
                        area += completed;
                        continue;
                    }

                    //Otherwise, start a new quad
                    faceList.add(face[0] + xOffset + yOffset + zOffset);
                    faceList.add(face[1]);
                    //Add merge info entry
                    mergeIndex = z;

                    thisRow[z][0] = face[0];
                    thisRow[z][1] = face[1];
                    thisRow[z][2] = 1;
                    thisRow[z][3] = faceList.size() - 1;
                    area ++;
                }
                int[][] temp = lastRow;
                lastRow = thisRow;
                thisRow = temp;
                mergeIndex = 0;
                for (int j = 0; j < 16; j ++) {
                    thisRow[j][0] = 0;
                    thisRow[j][1] = 0;
                    thisRow[j][2] = 0;
                    thisRow[j][3] = 0;
                }
            }
            for (int j = 0; j < 16; j ++) {
                lastRow[j][0] = 0;
                lastRow[j][1] = 0;
                lastRow[j][2] = 0;
                lastRow[j][3] = 0;
            }
        }
        this.counts[Face.UP.ordinal()] = faceList.size() * 5 / 2;
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
                for (int j = 0; j < 16; j ++) {
                    currentRow[j][0] = 0;
                    currentRow[j][1] = 0;
                    currentRow[j][2] = 0;
                    currentRow[j][3] = 0;
                }
            }
            for (int j = 0; j < 16; j ++) {
                lastRow[j][0] = 0;
                lastRow[j][1] = 0;
                lastRow[j][2] = 0;
                lastRow[j][3] = 0;
            }
        }
        this.counts[Face.SOUTH.ordinal()] = faceList.size() * 5 / 2;
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
