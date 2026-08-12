# This is free and unencumbered software released into the public domain.
#
# Anyone is free to copy, modify, publish, use, compile, sell, or
# distribute this software, either in source code form or as a compiled
# binary, for any purpose, commercial or non-commercial, and by any
# means.
#
# In jurisdictions that recognize copyright laws, the author or authors
# of this software dedicate any and all copyright interest in the
# software to the public domain. We make this dedication for the benefit
# of the public at large and to the detriment of our heirs and
# successors. We intend this dedication to be an overt act of
# relinquishment in perpetuity of all present and future rights to this
# software under copyright law.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
# EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
# MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
# IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR
# OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
# ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
# OTHER DEALINGS IN THE SOFTWARE.
#
# For more information, please refer to <https://unlicense.org>

all: test

test: libbase64.so.0 src/test.c
	${CC} -g -o test src/test.c -I../include -L. -lbase64 -Wl,-rpath,'$$ORIGIN'

libbase64.so.0:
	cd ..; cargo rustc --release --crate-type=cdylib --target-dir ${CURDIR}/rust_out ; find ${CURDIR}/rust_out -name libbase64.so -execdir cp '{}' ${CURDIR}/libbase64.so ';' ; rm -rf ${CURDIR}/rust_out
	ln -s libbase64.so libbase64.so.0

.PHONY: clean

clean:
	rm -f test
	rm -f libbase64.so
	rm -f libbase64.so.0
