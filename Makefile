build-go-lib:
	cd goproj && go build -buildmode c-archive -o libgolib.a ./golib.go