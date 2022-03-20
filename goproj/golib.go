package main

import "C"

//export GoFunction
func GoFunction(x int32) int64 {
	return int64(x * 2)
}

//export demoHello
func demoHello(name string) *C.char {
	return C.CString("Hello " + name)
}

func main() {

}
