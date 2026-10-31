# EDG C++ front end for the ILP32 target (x86 Linux, matching -m32).
# EDG's i686 configuration builds the front end itself as a 32-bit
# program, so the build stage needs a multilib toolchain.
ARG BASE=docker.io/library/ubuntu:24.04

FROM ${BASE} AS build
ARG EDG_REF=7.0
RUN apt-get update \
 && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
      ca-certificates git cmake ninja-build python3 g++ g++-multilib \
 && rm -rf /var/lib/apt/lists/*
RUN git clone --depth 1 --branch ${EDG_REF} https://github.com/edgcpp/compiler /edg
WORKDIR /edg
RUN cmake --preset linux-i686-gcc-debug -DCMAKE_BUILD_TYPE=Release -DEDG_BUILD_LIB=OFF \
 && cmake --build build/gcc-i686 -j"$(nproc)" --target cpfe

FROM ${BASE}
RUN dpkg --add-architecture i386 && apt-get update \
 && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends libc6:i386 libstdc++6:i386 \
 && rm -rf /var/lib/apt/lists/*
COPY --from=build /edg/build/gcc-i686/bin/cpfe /opt/edg/bin/cpfe
# Target configuration (predefined macros, type sizes) for x86 Linux.
COPY --from=build /edg/bases/docker/dev-env/gcc-i686 /opt/edg/base
ENV EDG_BASE=/opt/edg/base PATH=/opt/edg/bin:$PATH
