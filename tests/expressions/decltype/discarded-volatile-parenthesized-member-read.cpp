// tidal-rules: UNEV-2 UNEV-6
// tidal-expect: reject
// tidal-error-gcc: conversion of a volatile lvalue
// tidal-error-clang: read of volatile-qualified type
// tidal-error-edg: attempt to read from volatile storage
// tidal-note: RV12-1. The discarded expression is a pointer-to-member access in parentheses, and it refers to the member level of the volatile object g, so the program reads the member (UNEV-2). Constant evaluation rejects the read (UNEV-6).
struct Gauge { int level; };
constexpr int probe() {
  volatile Gauge g{1};
  int Gauge::*field = &Gauge::level;
  (g.*field);
  return 0;
}
static_assert(probe() == 0);
