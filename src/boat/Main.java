package boat;

import boat.util.GlHelper;
import org.lwjgl.opengl.GLUtil;
import org.lwjgl.system.MemoryStack;
import boat.world.Chunk;
import boat.world.World;

import java.io.IOException;
import java.nio.ByteBuffer;
import java.nio.FloatBuffer;
import java.nio.IntBuffer;
import java.util.Arrays;
import java.util.Random;

import static org.lwjgl.opengl.GL46C.*;
import static boat.util.FileHelper.readImage;
import static boat.util.FileHelper.readText;
import static boat.util.GlHelper.createProgram;
import static boat.util.GlHelper.createShader;

public class Main implements Runnable {
    public World world;
    public Camera camera;
    public Window window;

    public Main() throws IOException {
        this.camera = new Camera();
        this.window = new Window(this.camera, 600, 600);
        initGl();
        this.world = new World(this.window, true);
    }

    public void run() {
        this.window.world = this.world;
        do {
            this.window.processKeys(this.camera, this.world);
            this.draw();
        } while (this.window.update());
        this.window.close();
    }

    private void initGl() throws IOException {
//        glEnable(GL_DEBUG_OUTPUT);
//        GLUtil.setupDebugMessageCallback();

        int vertShader = createShader(GL_VERTEX_SHADER, readText("src/boat/shader/shader.vert"));
        int fragShader = createShader(GL_FRAGMENT_SHADER, readText("src/boat/shader/shader.frag"));
        int program = createProgram(vertShader, fragShader);

        glUseProgram(program);

        glEnable(GL_CULL_FACE);
        glEnable(GL_DEPTH_TEST);

        glClearColor(1.0f, 1.0f, 1.0f, 0.0f);

        int texture = glGenTextures();
        glBindTexture(GL_TEXTURE_2D_ARRAY, texture);
        glTextureStorage3D(texture, 1, GL_RGB8, 16, 16, 4);

        int k = 0;
        int[] image = readImage("assets/grass_side.png");
        glTexSubImage3D(GL_TEXTURE_2D_ARRAY, 0, 0, 0, k ++, 16, 16, 1, GL_RGBA, GL_UNSIGNED_INT_8_8_8_8, image);

        image = readImage("assets/clay.png");
        glTexSubImage3D(GL_TEXTURE_2D_ARRAY, 0, 0, 0, k ++, 16, 16, 1, GL_RGBA, GL_UNSIGNED_INT_8_8_8_8, image);

        image = readImage("assets/diamond_ore.png");
        glTexSubImage3D(GL_TEXTURE_2D_ARRAY, 0, 0, 0, k ++, 16, 16, 1, GL_RGBA, GL_UNSIGNED_INT_8_8_8_8, image);

        image = readImage("assets/furnace_side.png");
        glTexSubImage3D(GL_TEXTURE_2D_ARRAY, 0, 0, 0, k ++, 16, 16, 1, GL_RGBA, GL_UNSIGNED_INT_8_8_8_8, image);

        image = readImage("assets/cobblestone.png");
        glTexSubImage3D(GL_TEXTURE_2D_ARRAY, 0, 0, 0, k ++, 16, 16, 1, GL_RGBA, GL_UNSIGNED_INT_8_8_8_8, image);

        image = readImage("assets/crafting_table_side.png");
        glTexSubImage3D(GL_TEXTURE_2D_ARRAY, 0, 0, 0, k ++, 16, 16, 1, GL_RGBA, GL_UNSIGNED_INT_8_8_8_8, image);


        glTexParameteri(GL_TEXTURE_2D_ARRAY, GL_TEXTURE_WRAP_S, GL_REPEAT);
        glTexParameteri(GL_TEXTURE_2D_ARRAY, GL_TEXTURE_WRAP_T, GL_REPEAT);
        glTexParameteri(GL_TEXTURE_2D_ARRAY, GL_TEXTURE_WRAP_R, GL_REPEAT);
        glTexParameteri(GL_TEXTURE_2D_ARRAY, GL_TEXTURE_MIN_FILTER, GL_NEAREST);
        glTexParameteri(GL_TEXTURE_2D_ARRAY, GL_TEXTURE_MAG_FILTER, GL_NEAREST);

        glEnable(GL_PRIMITIVE_RESTART);
        glPrimitiveRestartIndex(Integer.MAX_VALUE);
        int[] elementArray = new int[1024 * 1024 / 4];
        for (int i = 0, j = 0; i <= 1024 * 1024 / 4 - 1; i ++) {
            if (i % 5 == 4) elementArray[i] = Integer.MAX_VALUE;
            else elementArray[i] = (j ++);
        }
        glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, glGenBuffers());
        glBufferData(GL_ELEMENT_ARRAY_BUFFER, elementArray, GL_STATIC_DRAW);
    }

    private void draw() {
        if (this.world == null) return;
        glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

        try (MemoryStack stack = MemoryStack.create(16 * Float.BYTES).push()) {
            FloatBuffer buffer = this.camera.getMatrix().get(stack.mallocFloat(16));
            glUniformMatrix4fv(0, false, buffer);
        }

        for (int x = 0; x < this.world.chunkX; x++) {
            for (int y = 0; y < this.world.chunkY; y++) {
                for (int z = 0; z < this.world.chunkZ; z++) {
                    Chunk chunk = this.world.get(x, y, z);
                    if (chunk == null || !chunk.doneMeshing) return;
                    glBindBufferBase(GL_SHADER_STORAGE_BUFFER, 0, chunk.buffer);
//                    glDrawArrays(GL_TRIANGLE_STRIP, window.counter, 1);
//                    glDrawElements(GL_TRIANGLE_STRIP, chunk.faceCount * 5, GL_UNSIGNED_INT, 0);
                    glMultiDrawElements(GL_TRIANGLE_STRIP, chunk.countBuffer, GL_UNSIGNED_INT, chunk.offsetBuffer);
//                    glMultiDrawElements(GL_TRIANGLE_STRIP, chunk.counts, GL_UNSIGNED_INT, chunk.offsetBuffer);
                }
            }
        }
    }


    public static void main(String[] args) throws IOException {
//        IntBuffer buffer = ByteBuffer.allocateDirect(256 * 4).asIntBuffer();
//        int[] array = new int[256];
//        int[] counts = new int[16];
//        int start = 0;
//        int end = 256;
//        Random random = new Random();
//        for (int i = 0; i < array.length; i ++) {
//            int n = random.nextInt(0, 16);
//            array[i] = n;
//            buffer.put(n);
//        }
//
//        for (int i = start; i < end; i ++) {
//            counts[array[i] & 0xf] ++;
//        }
//        GlHelper.sortBuffer(buffer, array, counts, start, end);
//        System.out.println(Arrays.toString(array));
//        buffer.position(0);
//        int[] result = new int[256];
//        buffer.get(result);
//        System.out.println(Arrays.toString(result));
        new Main().run();

    }


}