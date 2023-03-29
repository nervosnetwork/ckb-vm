set -ex

if [ ! -d ./riscv-isa-sim ]; then
    git clone https://github.com/riscv-software-src/riscv-isa-sim
    cd riscv-isa-sim
    git checkout e6a2245b
    cd -

    sed -i '/softfloat_install_shared_lib/d' riscv-isa-sim/softfloat/softfloat.mk.in
    sed -i 's/private:/public:/g' riscv-isa-sim/riscv/processor.h
    sed -i 's/private:/public:/g' riscv-isa-sim/riscv/mmu.h

    mkdir -p riscv-isa-sim/build
    cd riscv-isa-sim/build
    ../configure CXX=clang++ CC=clang CFLAGS="-g -O1" CXXFLAGS="-g -O1"
    make -j$(nproc)
    cd -
fi

mkdir -p target
clang++ -c -fPIC -I./riscv-isa-sim/riscv  -I./riscv-isa-sim/build -I./riscv-isa-sim/softfloat -I./riscv-isa-sim/fesvr src/spike-interfaces.cc -o target/spike-interfaces.o
ar rcs target/libspike-interfaces.a target/spike-interfaces.o
