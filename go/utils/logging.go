package utils

import "fmt"

const DEBUG = 10
const INFO = 20
const WARN = 30
const ERROR = 40

var logLevel = INFO

func Debug(msg string, args ...any) {
	if logLevel > DEBUG {
		return
	}
	fmt.Println("DEBUG | " + fmt.Sprintf(msg, args...))
}

func Info(msg string, args ...any) {
	if logLevel > INFO {
		return
	}
	fmt.Println("INFO  | " + fmt.Sprintf(msg, args...))
}

func Warn(msg string, args ...any) {
	if logLevel > WARN {
		return
	}
	fmt.Println("WARN  | " + fmt.Sprintf(msg, args...))
}

func Error(msg string, args ...any) {
	if logLevel > ERROR {
		return
	}
	fmt.Println("ERROR | " + fmt.Sprintf(msg, args...))
}

func SetLogLevel(newLevel int) {
	logLevel = newLevel
}
