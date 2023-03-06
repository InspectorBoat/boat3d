package src;

import src.block.Blocks;
import org.lwjgl.glfw.GLFWErrorCallback;
import org.lwjgl.opengl.GL;
import src.world.World;

import static org.lwjgl.glfw.Callbacks.glfwFreeCallbacks;
import static org.lwjgl.glfw.GLFW.*;
import static org.lwjgl.opengl.GL11C.*;
import static org.lwjgl.system.MemoryUtil.NULL;

public class Window {
    public final long windowId;
    public World world;
    public int counter;
    private boolean maximized;
    private double prevMouseX = Double.NEGATIVE_INFINITY;
    private double prevMouseY = Double.NEGATIVE_INFINITY;
    private boolean cursorCaptured;
    private long frameCounter;
    private long startTime;
    private boolean mesh;

    public Window(Camera camera, int startWidth, int startHeight) {
        GLFWErrorCallback.createPrint(System.out).set();

        if (!glfwInit()) throw new IllegalStateException("Unable to initialize GLFW");

        glfwWindowHint(GLFW_VISIBLE, GLFW_TRUE);
        glfwWindowHint(GLFW_RESIZABLE, GLFW_TRUE);

        this.windowId = glfwCreateWindow(startWidth, startHeight, "", NULL, NULL);
        if (this.windowId == NULL) throw new RuntimeException("Failed to create the GLFW window");

        camera.setScale(startWidth, startHeight);

        glfwSetMouseButtonCallback(this.windowId, (window, button, action, mods) -> {
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
        glfwSetKeyCallback(this.windowId, (window, key, scancode, action, mods) -> {
            if (action != GLFW_PRESS) return;

            if (key == GLFW_KEY_TAB) {
                if (this.maximized) glfwRestoreWindow(window);
                else glfwMaximizeWindow(window);
            }
            else if (key == GLFW_KEY_ESCAPE) {
                glfwSetWindowShouldClose(window, true);
            }
            else if (key == GLFW_KEY_X) {
                glPolygonMode(GL_FRONT_AND_BACK, this.mesh ? GL_FILL : GL_LINE);
                this.mesh = !this.mesh;
            }
            else if (key == GLFW_KEY_F) {
                this.counter += 1;
//                this.world.genChunk(0, 0, 0);
//                this.world.meshChunk(0, 0, 0);
            }

        });
        glfwSetWindowMaximizeCallback(this.windowId, (window, maximized) -> this.maximized = maximized);
        glfwSetWindowSizeCallback(this.windowId, (window, width, height) -> {
            glViewport(0, 0, width, height);
            camera.setScale(width, height);
        });
        glfwSetCursorPosCallback(this.windowId, (window, x, y) -> {
            if (!this.cursorCaptured) return;
            if (this.prevMouseX != Double.NEGATIVE_INFINITY && this.prevMouseY != Double.NEGATIVE_INFINITY)
                camera.addAngle((float) (this.prevMouseX - x) / 500, (float) (this.prevMouseY - y) / 500);
            this.prevMouseX = x;
            this.prevMouseY = y;
        });

        glfwShowWindow(this.windowId);
        glfwMakeContextCurrent(this.windowId);
        glfwSwapInterval(1);

        GL.createCapabilities();
    }

    public void processKeys(Camera camera, World world) {
        float baseSpeed = 0.01f;
        int speedMult = (glfwGetKey(this.windowId, GLFW_KEY_LEFT_CONTROL) + 1) * 10;
        if (glfwGetKey(this.windowId, GLFW_KEY_A) == GLFW_PRESS) {
            camera.stepPos(-baseSpeed * speedMult, 0);
        }
        if (glfwGetKey(this.windowId, GLFW_KEY_D) == GLFW_PRESS) {
            camera.stepPos(baseSpeed * speedMult, 0);
        }
        if (glfwGetKey(this.windowId, GLFW_KEY_W) == GLFW_PRESS) {
            camera.stepPos(0, baseSpeed * speedMult);
        }
        if (glfwGetKey(this.windowId, GLFW_KEY_S) == GLFW_PRESS) {
            camera.stepPos(0, -baseSpeed * speedMult);
        }
        if (glfwGetKey(this.windowId, GLFW_KEY_SPACE) == GLFW_PRESS) {
            camera.addPos(0, baseSpeed * speedMult, 0);
        }
        if (glfwGetKey(this.windowId, GLFW_KEY_LEFT_SHIFT) == GLFW_PRESS) {
            camera.addPos(0, -baseSpeed * speedMult, 0);
        }
        if (glfwGetKey(this.windowId, GLFW_KEY_C) == GLFW_PRESS) {
            camera.fov = 20;
        } else camera.fov = 70;
        if (glfwGetKey(this.windowId, GLFW_KEY_R) == GLFW_PRESS) {
            System.out.println(camera.getMatrix());
            int x = (int) (camera.x / 16);
            int y = (int) (camera.y / 16);
            int z = (int) (camera.z / 16);
            for (int i = 0; i < 400; i ++) {
//                world.get(x, y, z).blocks[(int) (Math.random() * 4096)] = Blocks.states[Math.random() > 0.5 ? 1 : 0];
            }
//            world.genChunk(x, y, z);
//            long start = System.nanoTime();
//            world.meshChunk(x, y, z);
//            System.out.println((System.nanoTime() - start) / 1000000d);
        }
    }

    public boolean update() {
        glfwSwapBuffers(this.windowId);

        if (this.frameCounter >= 50) {
            long time = System.nanoTime();
//            System.out.println("FPS: " + 1000000000 / ((time - startTime) / frameCounter));
            if (glfwGetKey(this.windowId, GLFW_KEY_F) == GLFW_PRESS) {
                this.counter++;
                world.meshChunk(0, 0, 0);
            }
            this.startTime = time;
            frameCounter = 0;
        }
        else frameCounter++;

        glfwPollEvents();

        return !glfwWindowShouldClose(this.windowId);
    }


    public void close() {
        glfwFreeCallbacks(this.windowId);
        glfwDestroyWindow(this.windowId);
        glfwTerminate();
    }
}
