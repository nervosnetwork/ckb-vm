BUILDER_DOCKER="nervos/ckb-riscv-gnu-toolchain:bionic-20210727"

docker run --rm -v `pwd`:/code ${BUILDER_DOCKER} bash -c "cd /code && sh _build_all_native.sh"
