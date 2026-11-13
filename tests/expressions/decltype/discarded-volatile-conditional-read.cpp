// tidal-rules: UNEV-2 UNEV-6
// tidal-expect: reject
// tidal-error-gcc: conversion of a volatile lvalue
// tidal-error-clang: read of volatile-qualified type
// tidal-error-edg: attempt to read from volatile storage
// tidal-note: RV12-1. The discarded conditional expression has the names v and w as its second and third operands, so the program reads the volatile object that the result refers to (UNEV-2). Constant evaluation rejects the read (UNEV-6).
constexpr int pick() {
  volatile int v = 1;
  volatile int w = 2;
  true ? v : w;
  return 0;
}
static_assert(pick() == 0);
