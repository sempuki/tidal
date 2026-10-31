// tidal-rules: VOL-8
// tidal-expect: reject
// tidal-error-clang: not an integral constant expression
// tidal-error-gcc: conversion of a volatile lvalue
// tidal-error-edg: must have a constant value
constexpr int f() { volatile int v = 1; return v; }
static_assert(f() == 1);
