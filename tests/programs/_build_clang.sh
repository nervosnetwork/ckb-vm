set -ex

DOCKER="${DOCKER:-docker}"
# docker pull docker.io/cryptape/llvm-n-rust:20240630
DOCKER_IMAGE="${DOCKER_IMAGE:-docker.io/cryptape/llvm-n-rust@sha256:bafaf76d4f342a69b8691c08e77a330b7740631f3d1d9c9bee4ead521b29ee55}"

$DOCKER run --rm -e UID=`id -u` -e GID=`id -g` $DOCKER_RUN_ARGS -v `pwd`:/code $DOCKER_IMAGE bash _build_clang_native.sh
