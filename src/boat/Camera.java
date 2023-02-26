package boat;

import org.joml.Matrix4f;

public class Camera {
    public double x = 0;
    public double y = 0;
    public double z = 0;
    public double fov = 70;
    public float yaw = (float) (-Math.PI / 4) * 0;
    public float pitch = (float) (Math.PI / 4) * 0;

    public float width = 0;
    public float height = 0;

    private final Matrix4f cameraMatrix = new Matrix4f();



    public void setScale(float scaleX, float scaleY) {
        this.width = scaleX;
        this.height = scaleY;
        this.updateMatrix();
    }

    public void setPos(double x, double y, double z) {
        this.x = x;
        this.y = y;
        this.z = z;
        this.updateMatrix();
    }

    public void addPos(double x, double y, double z) {
        this.x += x;
        this.y += y;
        this.z += z;
        this.updateMatrix();
    }

    public void stepPos(double x, double z) {
        this.x += Math.cos(this.yaw) * x - Math.sin(this.yaw) * z;
        this.z += Math.sin(this.yaw) * x + Math.cos(this.yaw) * z;
        this.updateMatrix();
    }

    public void setAngle(float yaw, float pitch) {
        this.yaw = yaw;
        this.pitch = pitch;
        this.updateMatrix();
    }

    public void addAngle(float yaw, float pitch) {
        this.yaw += yaw;
        this.pitch += pitch;
        this.updateMatrix();
    }

    private void updateMatrix() {

    }

    public Matrix4f getMatrix() {
        this.cameraMatrix
                .identity()
                .perspective((float) (this.fov / 180 * Math.PI), this.width / this.height, 0.1f, Float.POSITIVE_INFINITY)
                .scale(-1, 1, 1)
                .rotateX(-this.pitch)
                .rotateY((float) (this.yaw + Math.PI))
                .translate((float) -this.x, (float) -this.y, (float) -this.z)
                .scale(1f / 16, 1f / 16, 1f / 16)
        ;
        return this.cameraMatrix;
    }
}
