// tidal-rules: UNEV-2 UNEV-6
// tidal-expect: reject
// tidal-error-gcc: conversion of a volatile lvalue
// tidal-error-clang: read of volatile-qualified type
// tidal-error-edg: attempt to read from volatile storage
// tidal-note: RV12-1. The right operand of the discarded comma expression is the name v, so the program reads the volatile object (UNEV-2). Constant evaluation rejects the read (UNEV-6).
constexpr int last() {
  volatile int v = 1;
  (void)0, v;
  return 0;
}
static_assert(last() == 0);
