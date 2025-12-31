package adventofcode.util;

public class Log {
    public enum Level {
        DEBUG(10),
        INFO(20),
        WARN(30),
        ERROR(40);

        private final int value;

        Level(int value) {
            this.value = value;
        }

        public int value() {
            return value;
        }
    }

    private static Level configuredLevel = Level.INFO;

    public static void setLogLevel(Level level) {
        configuredLevel = level;
    }

    public static void debug(String msg, Object... args) {
        doLog(Level.DEBUG, msg, args);
    }

    public static void info(String msg, Object... args) {
        doLog(Level.INFO, msg, args);
    }

    public static void warn(String msg, Object... args) {
        doLog(Level.WARN, msg, args);
    }

    public static void error(String msg, Object... args) {
        doLog(Level.ERROR, msg, args);
    }

    private static void doLog(Level level, String msg, Object[] args) {
        if (configuredLevel.value() <= level.value()) {
            System.out.println(level.name() + " | " + String.format(msg, args));
        }
    }
}
