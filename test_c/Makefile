CWD != pwd

all: test

test: libbase64.a src/test.c
	${CC} -g -o test src/test.c -I../include -L. -lbase64

libbase64.a:
	cd ..; cargo build --release --target-dir ${CWD}/rust_out ; find ${CWD}/rust_out -name libbase64.a -execdir cp '{}' ${CWD}/libbase64.a ';' ; rm -rf ${CWD}/rust_out

.PHONY: clean

clean:
	rm -f test
	rm -f libbase64.a
