package util;

import javax.imageio.ImageIO;
import java.awt.image.BufferedImage;
import java.awt.image.DataBufferByte;
import java.io.File;
import java.io.FileInputStream;
import java.io.IOException;
import java.nio.charset.StandardCharsets;

public class FileHelper {
    private FileHelper() {}

    public static String readText(String path) throws IOException {
        File file = new File(path);
        FileInputStream stream = new FileInputStream(file);
        byte[] data = new byte[(int) file.length()];
        //noinspection ResultOfMethodCallIgnored
        stream.read(data);
        stream.close();

        return new String(data, StandardCharsets.UTF_8);
    }

    public static int[] readImage(String path) throws IOException {
        BufferedImage img = ImageIO.read(new File(path));
        byte[] data = ((DataBufferByte) img.getRaster().getDataBuffer()).getData();
        int[] pixels = new int[data.length / 4];
        try {
            Thread.sleep(1000);
        } catch (InterruptedException e) {
            throw new RuntimeException(e);
        }
        for (int i = 0; i < pixels.length; i++) {
            pixels[i] |= (((int) data[i * 4 + 3]) & 0xff) << 24;
            pixels[i] |= (((int) data[i * 4 + 2]) & 0xff) << 16;
            pixels[i] |= (((int) data[i * 4 + 1]) & 0xff) << 8;
        }
        return pixels;
    }
}
