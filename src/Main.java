import org.lwjgl.opengl.GLUtil;
import org.lwjgl.system.MemoryStack;
import world.Chunk;
import world.World;

import java.io.IOException;
import java.nio.FloatBuffer;

import static org.lwjgl.opengl.GL46C.*;
import static util.FileHelper.readText;
import static util.GlHelper.createProgram;
import static util.GlHelper.createShader;

public class Main {
    public World world;
    public Camera camera;
    public Window window;

    public Main() throws IOException {
        this.camera = new Camera();
        this.window = new Window(this.camera, 600, 600);
        initGl();

        this.world = new World();
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

        int vertShader = createShader(GL_VERTEX_SHADER, readText("src/shader/shader.vert"));
        int fragShader = createShader(GL_FRAGMENT_SHADER, readText("src/shader/shader.frag"));
        int program = createProgram(vertShader, fragShader);

        glUseProgram(program);

        glEnable(GL_CULL_FACE);
        glEnable(GL_DEPTH_TEST);
        glClearColor(1.0f, 1.0f, 1.0f, 0.0f);

        int VAO = glGenVertexArrays();
        glBindVertexArray(VAO);

        glVertexAttribFormat(0, 3, GL_SHORT, false, 0);
        glVertexAttribBinding(0, 0);
        glEnableVertexAttribArray(0);

        glVertexAttribFormat(1, 1, GL_UNSIGNED_SHORT, true, 2 * 3);
        glVertexAttribBinding(1, 0);
        glEnableVertexAttribArray(1);

        int texture = glGenTextures();
        glBindTexture(GL_TEXTURE_2D_ARRAY, texture);
        glTextureStorage3D(texture, 1, GL_RGB8, 2, 2, 1024);
        glTexSubImage3D(GL_TEXTURE_2D_ARRAY, 0, 0, 0, 0, 2, 2, 2, GL_RGBA, GL_UNSIGNED_INT_8_8_8_8, new int[] {
                0xff0000ff, 0x00ffffff,
                0x00ff00ff, 0xaaaaaaff,
                0x00ffffff, 0xaaaaaaff,
                0x00fa00ff, 0xaa2aaaff,
        });
        glTexParameteri(GL_TEXTURE_2D_ARRAY, GL_TEXTURE_MIN_FILTER, GL_NEAREST);
        glTexParameteri(GL_TEXTURE_2D_ARRAY, GL_TEXTURE_MAG_FILTER, GL_NEAREST);

    }


    private void draw() {
        glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

        try (MemoryStack stack = MemoryStack.stackPush()) {
            FloatBuffer buffer = this.camera.getMatrix().get(stack.mallocFloat(16));
            glUniformMatrix4fv(0, false, buffer);
        }

        for (int x = 0; x < this.world.chunkX; x++) {
            for (int y = 0; y < this.world.chunkY; y++) {
                for (int z = 0; z < this.world.chunkZ; z++) {
                    Chunk chunk = this.world.get(x, y, z);
                    glUniform4f(1, x * 16, y * 16, z * 16, 0);
                    glBindVertexBuffer(0, chunk.buffer, 0, 2 * 4);
                    glPrimitiveRestartIndex(1);
                    glDrawArrays(GL_QUADS, 0, chunk.vertexCount);
                }
            }
        }
    }


    public static void main(String[] args) throws IOException {
        new Main().run();
    }

}