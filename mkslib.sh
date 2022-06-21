#!/bin/sh -e
echo ${DESTDIR?'!'} >/dev/null 2>&1
cd $(dirname $(readlink -fn "${0}"))
cc -xc -std=c99 -w -Ofast -O2 \
    -D_XOPEN_SOURCE -D_DEFAULT_SOURCE \
        -mtune=generic -DTB_IMPL -fPIC -c \
            termbox2.c -o termbox2.o

if [ ! -d "${DESTDIR}/lib" ]
then install -d "${DESTDIR}/lib"
fi

ar rcs "${DESTDIR}/lib/libtermbox2.a" termbox2.o >/dev/null 2>&1

rm -f termbox2.o

######
exit 0
