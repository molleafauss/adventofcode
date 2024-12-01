package aoc

import "fmt"

const DEBUG = 10
const INFO = 20
const ERROR = 30

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

func Error(msg string, args ...any) {
	if logLevel > ERROR {
		return
	}
	fmt.Println("ERROR | " + fmt.Sprintf(msg, args...))
}

func SetLogLevel(newLevel int) {
	logLevel = newLevel
}
