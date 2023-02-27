package boat.util;

public class DebugHelper {
    public static boolean log = true;
    public static void log(Object msg, Object ...format) {
        if (log) System.out.printf(msg + "", format);
    }
    public static void logn(Object msg, Object ...format) {
        if (log) System.out.printf(msg + "\n", format);
    }
    public static void nolog() {
        log = false;
    }
    public static void dolog() {
        log = true;
    }
}
