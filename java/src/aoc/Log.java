package aoc;

public class Log {
    private static LogLevel configuredLogLevel = LogLevel.INFO;

    public static void setLogLevel(LogLevel logLevel) {
        configuredLogLevel = logLevel;
    }

    public static void debug(String msg, Object... args) {
        doLog(LogLevel.DEBUG, msg, args);
    }

    public static void info(String msg, Object... args) {
        doLog(LogLevel.INFO, msg, args);
    }

    public static void warn(String msg, Object... args) {
        doLog(LogLevel.WARN, msg, args);
    }

    public static void error(String msg, Object... args) {
        doLog(LogLevel.ERROR, msg, args);
    }

    private static void doLog(LogLevel logLevel, String msg, Object[] args) {
        if (configuredLogLevel.value() <= logLevel.value()) {
            System.out.println(logLevel.name() + " | " + String.format(msg, args));
        }
    }
}
