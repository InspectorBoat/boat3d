import org.lwjgl.Version;
import org.lwjgl.glfw.GLFWErrorCallback;
import org.lwjgl.opengl.GL;
import org.lwjgl.system.MemoryStack;
import world.Chunk;
import world.World;

import java.io.IOException;
import java.nio.FloatBuffer;

import static org.lwjgl.glfw.Callbacks.glfwFreeCallbacks;
import static org.lwjgl.glfw.GLFW.*;
import static org.lwjgl.opengl.GL46.*;
import static org.lwjgl.system.MemoryUtil.NULL;
import static util.FileHelper.readText;
import static util.GlHelper.createProgram;
import static util.GlHelper.createShader;

public class Main {
    private long window;
    public World world = new World();
    private double prevMouseX = Double.NEGATIVE_INFINITY;
    private double prevMouseY = Double.NEGATIVE_INFINITY;
    public Camera camera = new Camera();
    public boolean cursorCaptured = false;

    public int chunkX = 4;
    public int chunkY = 4;
    public int chunkZ = 4;
    public boolean maximized;

    public void run() throws IOException
    {
        System.out.println("Hello LWJGL " + Version.getVersion() + "!");
        initWindow();
        initGl();
        initWorld();

        loop();

        glfwFreeCallbacks(window);
        glfwDestroyWindow(window);
        glfwTerminate();
    }

    public void initWorld()
    {
        glEnableVertexAttribArray(0);
        for (int x = 0; x < chunkX; x++) {
            for (int y = 0; y < chunkY; y++) {
                for (int z = 0; z < chunkZ; z++) {
                    this.world.genChunk(x, y, z);
                }
            }
        }
    }

    private void initWindow()
    {
        GLFWErrorCallback.createPrint(System.out).set();

        if (!glfwInit()) throw new IllegalStateException("Unable to initialize GLFW");

        glfwWindowHint(GLFW_VISIBLE, GLFW_TRUE);
        glfwWindowHint(GLFW_RESIZABLE, GLFW_TRUE);

        window = glfwCreateWindow(600, 600, "Hello World!", NULL, NULL);
        if (window == NULL) throw new RuntimeException("Failed to create the GLFW window");
        camera.setScale(600, 600);

        glfwSetMouseButtonCallback(window, (window, button, action, mods) -> {
            if (action != GLFW_PRESS) return;
            if (button == GLFW_MOUSE_BUTTON_1) {
                glfwSetInputMode(window, GLFW_CURSOR, GLFW_CURSOR_DISABLED);
                this.cursorCaptured = true;
            }
            else if (button == GLFW_MOUSE_BUTTON_2) {
                if (this.cursorCaptured) {
                    glfwSetInputMode(window, GLFW_CURSOR, GLFW_CURSOR_NORMAL);
                    this.prevMouseX = Double.NEGATIVE_INFINITY;
                    this.prevMouseY = Double.NEGATIVE_INFINITY;
                    this.cursorCaptured = false;
                }
            }
        });
        glfwSetKeyCallback(window, (window, key, scancode, action, mods) -> {
            if (action != GLFW_PRESS) return;

            if (key == GLFW_KEY_TAB) {
                if (this.maximized) glfwRestoreWindow(window);
                else glfwMaximizeWindow(window);
            }
            else if (key == GLFW_KEY_ESCAPE) {
                glfwSetWindowShouldClose(window, true);
            }
        });
        glfwSetWindowMaximizeCallback(window, (window, maximized) -> this.maximized = maximized);
        glfwSetWindowSizeCallback(window, (window, width, height) -> {
            glViewport(0, 0, width, height);
            camera.setScale(width, height);
        });
        glfwSetCursorPosCallback(window, (window, x, y) -> {
            if (!this.cursorCaptured) return;
            if (this.prevMouseX != Double.NEGATIVE_INFINITY && this.prevMouseY != Double.NEGATIVE_INFINITY)
                this.camera.addAngle((float) (this.prevMouseX - x) / 500, (float) (this.prevMouseY - y) / 500);
            this.prevMouseX = x;
            this.prevMouseY = y;
        });

        glfwShowWindow(window);
        glfwMakeContextCurrent(window);
        glfwSwapInterval(1);

        GL.createCapabilities();
    }

    private void initGl() throws IOException
    {
        int vertShader = createShader(GL_VERTEX_SHADER, readText("src/shader/shader.vert"));
        int fragShader = createShader(GL_FRAGMENT_SHADER, readText("src/shader/shader.frag"));
        int program = createProgram(vertShader, fragShader);

        glUseProgram(program);

        glEnable(GL_CULL_FACE);
        glEnable(GL_DEPTH_TEST);
        glClearColor(1.0f, 1.0f, 1.0f, 0.0f);
    }

    private void draw()
    {
        try (MemoryStack stack = MemoryStack.stackPush()) {
            FloatBuffer buffer = this.camera.getMatrix().get(stack.mallocFloat(16));
            glUniformMatrix4fv(0, false, buffer);
        }

        for (int x = 0; x < chunkX; x++) {
            for (int y = 0; y < chunkY; y++) {
                for (int z = 0; z < chunkZ; z++) {
                    Chunk chunk = this.world.get(x, y, z);
                    glUniform4f(1, x * 16, y * 16, z * 16, 0);
                    glBindBuffer(GL_ARRAY_BUFFER, chunk.buffer);
                    glVertexAttribPointer(0, 3, GL_SHORT, false, 0, 0);
                    glDrawArrays(GL_TRIANGLES, 0, chunk.triangleCount);
                }
            }
        }
    }

    private void processKeys()
    {
        int speedMult = (glfwGetKey(window, GLFW_KEY_LEFT_CONTROL) + 1) * 10;
        if (glfwGetKey(window, GLFW_KEY_A) == GLFW_PRESS) {
            this.camera.stepPos(-0.03 * speedMult, 0);
        }
        if (glfwGetKey(window, GLFW_KEY_D) == GLFW_PRESS) {
            this.camera.stepPos(0.03 * speedMult, 0);
        }
        if (glfwGetKey(window, GLFW_KEY_W) == GLFW_PRESS) {
            this.camera.stepPos(0, 0.03 * speedMult);
        }
        if (glfwGetKey(window, GLFW_KEY_S) == GLFW_PRESS) {
            this.camera.stepPos(0, -0.03 * speedMult);
        }
        if (glfwGetKey(window, GLFW_KEY_SPACE) == GLFW_PRESS) {
            this.camera.addPos(0, 0.03 * speedMult, 0);
        }
        if (glfwGetKey(window, GLFW_KEY_LEFT_SHIFT) == GLFW_PRESS) {
            this.camera.addPos(0, -0.03 * speedMult, 0);
        }
        if (glfwGetKey(window, GLFW_KEY_R) == GLFW_PRESS) {
            this.world.regenChunk((int) (Math.random() * chunkX), (int) (Math.random() * chunkY), (int) (Math.random() * chunkZ));
        }
    }

    private void loop()
    {
        long startTime = System.nanoTime();
        double frameCounter = 1;
        while (!glfwWindowShouldClose(window)) {
            glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
            this.processKeys();
            this.draw();
            glfwSwapBuffers(window);

            if (frameCounter >= 1000) {
                long time = System.nanoTime();
                System.out.println("AVERAGE: " + 1000000000 / ((time - startTime) / frameCounter));
                startTime = time;
                frameCounter = 0;
            }
            else frameCounter++;

            glfwPollEvents();
        }
    }

    public static void main(String[] args) throws IOException
    {
        new Main().run();
    }

}