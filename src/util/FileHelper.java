package util;

import java.io.File;
import java.io.FileInputStream;
import java.io.IOException;
import java.nio.charset.StandardCharsets;

public class FileHelper {
    private FileHelper() {}
    public static String readText(String path) throws IOException
    {
        File file = new File(path);
        FileInputStream stream = new FileInputStream(file);
        byte[] data = new byte[(int) file.length()];
        stream.read(data);
        stream.close();

        return new String(data, StandardCharsets.UTF_8);
    }
}
