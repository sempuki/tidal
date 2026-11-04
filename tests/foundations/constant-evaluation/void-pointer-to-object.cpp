// tidal-rules: CE-4
// tidal-expect: reject
// tidal-error-gcc: cast from .void\*. is not allowed in a constant expression
// tidal-error-clang: cast from .void \*. is not allowed in a constant expression
// tidal-error-edg: conversion from .void \*. to .int \*. is invalid
constexpr int f() {
  int x = 7;
  void* v = &x;
  return *static_cast<int*>(v);
}
static_assert(f() == 7);
