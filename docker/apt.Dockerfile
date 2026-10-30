# A distribution compiler package, for testing a specific GCC or Clang version.
ARG BASE=docker.io/library/ubuntu:24.04
FROM ${BASE}
ARG PACKAGES
RUN apt-get update \
 && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends ${PACKAGES} \
 && rm -rf /var/lib/apt/lists/*
