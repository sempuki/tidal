// tidal-rules: CE-12
// tidal-expect: reject
// tidal-error-gcc: conversion of a volatile lvalue
// tidal-error-clang: read of volatile-qualified type
// tidal-error-edg: attempt to read from volatile storage
constexpr int f() {
  volatile int v = 1;
  return v;
}
static_assert(f() == 1);
