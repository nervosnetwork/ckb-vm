set -ex

ROOT_DIR=$(pwd)
CLANG="${CLANG:-clang-18}"
LD="${CLANG/clang/ld.lld}"
CFLAGS="--target=riscv64 -march=rv64imac_zba_zbb_zbc_zbs -nostdinc -isystem $ROOT_DIR/deps/musl/release/include -c -fdata-sections -ffunction-sections"
LDFLAGS="--gc-sections -nostdlib --sysroot $ROOT_DIR/deps/musl/release -L$ROOT_DIR/deps/musl/release/lib -lc -lgcc"

if [ ! -d deps ]; then
	mkdir deps
fi

if [ ! -d deps/musl ]; then
	cd deps
	git clone https://github.com/xxuejie/musl
	cd musl
	git checkout 603d5e9
	cd ../..
fi

if [ ! -d deps/musl/release ]; then
	cd deps/musl
	CLANG=$CLANG ./ckb/build.sh
	cd -
fi

$CLANG $CFLAGS clang_sample.c -o clang_sample.o && $LD $LDFLAGS clang_sample.o -o clang_sample && rm clang_sample.o
