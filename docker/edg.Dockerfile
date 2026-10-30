# EDG C++ front end, built from the open source release.
# Front end only: it checks programs but does not produce executables.
ARG BASE=docker.io/library/ubuntu:24.04
FROM ${BASE} AS build
ARG EDG_REF=7.0
RUN apt-get update \
 && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
      ca-certificates git cmake ninja-build python3 g++ \
 && rm -rf /var/lib/apt/lists/*
RUN git clone --depth 1 --branch ${EDG_REF} https://github.com/edgcpp/compiler /edg
WORKDIR /edg
# Only the front end (cpfe) is needed, not the runtime library.
RUN cmake --preset linux-gcc-release -DEDG_BUILD_LIB=OFF \
 && cmake --build build/gcc-release -j"$(nproc)"
