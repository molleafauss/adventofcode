package aoc;

public enum LogLevel {
    DEBUG(10),
    INFO(20),
    WARN(30),
    ERROR(40);

    private final int value;

    LogLevel(int value) {
        this.value = value;
    }

    public int value() {
        return value;
    }
}
