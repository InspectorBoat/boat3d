import org.lwjgl.opengl.GLUtil;
import org.lwjgl.system.MemoryStack;
import world.Chunk;
import world.World;

import java.io.IOException;
import java.nio.FloatBuffer;

import static org.lwjgl.opengl.GL46C.*;
import static util.FileHelper.readImage;
import static util.FileHelper.readText;
import static util.GlHelper.createProgram;
import static util.GlHelper.createShader;

public class Main implements Runnable {
    public World world;
    public Camera camera;
    public Window window;

    public Main() throws IOException {
        this.camera = new Camera();
        this.window = new Window(this.camera, 600, 600);
        initGl();
        this.world = new World(false);
    }

    public void run() {
        do {
            this.window.processKeys(this.camera, this.world);
            this.draw();
        } while (this.window.update());
        this.window.close();
    }

    private void initGl() throws IOException {
        glEnable(GL_DEBUG_OUTPUT);
        GLUtil.setupDebugMessageCallback();
        glEnable(GL_PROGRAM_POINT_SIZE);

        int vertShader = createShader(GL_VERTEX_SHADER, readText("src/shader/shader.vert"));
        int fragShader = createShader(GL_FRAGMENT_SHADER, readText("src/shader/shader.frag"));
        int program = createProgram(vertShader, fragShader);

        glUseProgram(program);

        glEnable(GL_CULL_FACE);
        glEnable(GL_DEPTH_TEST);

        glClearColor(1.0f, 1.0f, 1.0f, 0.0f);
        int[] test = new int[5002];
        for (int i = 4; i < 5002; i += 2) {
            test[i] = ((int) (Math.random() * 0xffd) << 24) | ((int) (Math.random() * 0xffd) << 16) | ((int) (Math.random() * 0xffd) << 8) | ((int) (Math.random() * 6d));
            test[i + 1] = ((int) (Math.random() * 0x8fd) << 24) | ((int) (Math.random() * 0x8fd) << 16) | 1;
        }
        glBindBufferBase(GL_SHADER_STORAGE_BUFFER, 1, glGenBuffers());
        glBufferData(GL_SHADER_STORAGE_BUFFER, test, GL_STATIC_DRAW);

        int texture = glGenTextures();
        glBindTexture(GL_TEXTURE_2D_ARRAY, texture);
        glTextureStorage3D(texture, 1, GL_RGB8, 16, 16, 2048);

        int[] image = readImage("assets/grass_side.png");
        glTexSubImage3D(GL_TEXTURE_2D_ARRAY, 0, 0, 0, 0, 16, 16, 1, GL_RGBA, GL_UNSIGNED_INT_8_8_8_8, image);
        glGenerateMipmap(GL_TEXTURE_2D_ARRAY);
        glTexParameteri(GL_TEXTURE_2D_ARRAY, GL_TEXTURE_MIN_FILTER, GL_LINEAR_MIPMAP_LINEAR);
        glTexParameteri(GL_TEXTURE_2D_ARRAY, GL_TEXTURE_MAG_FILTER, GL_NEAREST);

        glEnable(GL_PRIMITIVE_RESTART);
        glPrimitiveRestartIndex(Integer.MAX_VALUE);
        int[] elementArray = new int[100003];
        for (int i = 0, j = 0; i <= 100002; i ++) {
            if (i % 5 == 4) elementArray[i] = Integer.MAX_VALUE;
            else elementArray[i] = j ++;
        }
        glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, glGenBuffers());
        glBufferData(GL_ELEMENT_ARRAY_BUFFER, elementArray, GL_STATIC_DRAW);
    }

    private void draw() {
        if (this.world == null) return;
        glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

        try (MemoryStack stack = MemoryStack.stackPush()) {
            FloatBuffer buffer = this.camera.getMatrix().get(stack.mallocFloat(16));
            glUniformMatrix4fv(0, false, buffer);
        }
        glDrawElements(GL_TRIANGLE_STRIP, 100003, GL_UNSIGNED_INT, 0);
//        for (int x = 0; x < this.world.chunkX; x++) {
//            for (int y = 0; y < this.world.chunkY; y++) {
//                for (int z = 0; z < this.world.chunkZ; z++) {
//                    Chunk chunk = this.world.get(x, y, z);
//                    if (chunk == null || !chunk.doneMeshing) return;
//                    glUniform3i(1, x * 256, y * 256, z * 256);
//                    glBindVertexBuffer(0, chunk.buffer, 0, Byte.BYTES * 4);
//
//                    glDrawElements(GL_TRIANGLE_STRIP, chunk.vertexCount * 5 / 4, GL_UNSIGNED_INT, 0);
//                    glMultiDrawElements(GL_TRIANGLE_STRIP, chunk.countBuffer, GL_UNSIGNED_INT, chunk.offsetBuffer);
//                    glMultiDrawElements(GL_TRIANGLE_STRIP, chunk.counts, GL_UNSIGNED_INT, chunk.offsetBuffer);
//                }
//            }
//        }
    }


    public static void main(String[] args) throws IOException {
        new Main().run();
    }


}