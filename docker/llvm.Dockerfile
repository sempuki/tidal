# Clang from apt.llvm.org, for versions newer than the distribution ships.
ARG BASE=docker.io/library/ubuntu:24.04
FROM ${BASE}
ARG LLVM_VERSION
RUN apt-get update \
 && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
      ca-certificates curl gnupg \
 && curl -fsSL https://apt.llvm.org/llvm-snapshot.gpg.key \
      | gpg --dearmor -o /usr/share/keyrings/llvm.gpg \
 && . /etc/os-release \
 && echo "deb [signed-by=/usr/share/keyrings/llvm.gpg] http://apt.llvm.org/${VERSION_CODENAME}/ llvm-toolchain-${VERSION_CODENAME}-${LLVM_VERSION} main" \
      > /etc/apt/sources.list.d/llvm.list \
 && apt-get update \
 && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
      clang-${LLVM_VERSION} libclang-rt-${LLVM_VERSION}-dev g++-multilib \
 && rm -rf /var/lib/apt/lists/*
