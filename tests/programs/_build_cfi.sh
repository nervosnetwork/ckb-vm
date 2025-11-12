set -ex

DOCKER="${DOCKER:-docker}"
# TODO: update it to official docker image when CFI feature is stable
DOCKER_IMAGE="${DOCKER_IMAGE:-docker.io/xujiandong/ckb-riscv-llvm-toolchain@sha256:a1b17f23fa013e139f048faefded6b579122dd32c58cf86cdfe88fc3aff4397e}"

$DOCKER run --rm -e UID=`id -u` -e GID=`id -g` $DOCKER_RUN_ARGS -v `pwd`:/code $DOCKER_IMAGE bash -c "cd code && bash _build_cfi_native.sh"
