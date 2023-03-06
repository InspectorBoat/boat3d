package src.util;

import java.util.Arrays;

public class DebugHelper {
    public static boolean log = true;

    public static void log(Object ...msg) {
        if (log) {
            if (msg.length == 1) System.out.print(msg[0]);
            else System.out.print(Arrays.toString(msg));
        }
    }

    public static void logn(Object ...msg) {
        if (log) {
            if (msg.length == 1) System.out.println(msg[0]);
            else System.out.println(Arrays.toString(msg));
        }
    }

    public static void logf(Object msg, Object ...format) {
        if (log) System.out.printf(msg + "", format);
    }
    public static void logfn(Object msg, Object ...format) {
        if (log) System.out.printf(msg + "\n", format);
    }
    public static void nolog() {
        log = false;
    }
    public static void dolog() {
        log = true;
    }
}
