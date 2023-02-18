import org.lwjgl.glfw.GLFWErrorCallback;
import org.lwjgl.opengl.GL;
import world.World;

import static org.lwjgl.glfw.Callbacks.glfwFreeCallbacks;
import static org.lwjgl.glfw.GLFW.*;
import static org.lwjgl.opengl.GL11C.glViewport;
import static org.lwjgl.system.MemoryUtil.NULL;

public class Window {
    private final long windowId;
    private boolean maximized;
    private double prevMouseX;
    private double prevMouseY;
    private boolean cursorCaptured;
    private long frameCounter;
    private long startTime;

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
        int speedMult = (glfwGetKey(this.windowId, GLFW_KEY_LEFT_CONTROL) + 1) * 10;
        if (glfwGetKey(this.windowId, GLFW_KEY_A) == GLFW_PRESS) {
            camera.stepPos(-0.03 * speedMult, 0);
        }
        if (glfwGetKey(this.windowId, GLFW_KEY_D) == GLFW_PRESS) {
            camera.stepPos(0.03 * speedMult, 0);
        }
        if (glfwGetKey(this.windowId, GLFW_KEY_W) == GLFW_PRESS) {
            camera.stepPos(0, 0.03 * speedMult);
        }
        if (glfwGetKey(this.windowId, GLFW_KEY_S) == GLFW_PRESS) {
            camera.stepPos(0, -0.03 * speedMult);
        }
        if (glfwGetKey(this.windowId, GLFW_KEY_SPACE) == GLFW_PRESS) {
            camera.addPos(0, 0.03 * speedMult, 0);
        }
        if (glfwGetKey(this.windowId, GLFW_KEY_LEFT_SHIFT) == GLFW_PRESS) {
            camera.addPos(0, -0.03 * speedMult, 0);
        }
        if (glfwGetKey(this.windowId, GLFW_KEY_R) == GLFW_PRESS) {
            world.regenChunk((int) (Math.random() * world.chunkX), (int) (Math.random() * world.chunkY), (int) (Math.random() * world.chunkZ));
        }
    }

    public boolean update() {
        glfwSwapBuffers(this.windowId);

        if (this.frameCounter >= 50) {
            long time = System.nanoTime();
            System.out.println("AVERAGE: " + 1000000000 / ((time - startTime) / frameCounter));
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
