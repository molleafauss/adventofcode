namespace adventofcode.utils;

using System;

public enum LogLevel
{
    Debug = 0,
    Info = 1,
    Warn = 2,
    Error = 3,
    None = 4
}

public static class Log
{
    private static LogLevel _minLevel = LogLevel.Info;

    public static void SetLevel(LogLevel level) => _minLevel = level;

    public static void Debug(string message) => _log(LogLevel.Debug, message);
    public static void Info(string message) => _log(LogLevel.Info, message);
    public static void Warn(string message) => _log(LogLevel.Warn, message);
    public static void Error(string message) => _log(LogLevel.Error, message);

    private static void _log(LogLevel level, string message)
    {
        if (level < _minLevel || _minLevel == LogLevel.None) return;

        var levelName = level.ToString().ToUpperInvariant();
        Console.WriteLine($"[{levelName}] {message}");
    }
}
