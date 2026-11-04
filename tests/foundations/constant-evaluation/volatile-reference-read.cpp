// tidal-rules: CE-12
// tidal-expect: reject
// tidal-error-clang: read of volatile-qualified type
// tidal-error-gcc: conversion of a volatile lvalue
// tidal-error-edg: attempt to read from volatile storage
// tidal-note: The object is not volatile, but the evaluation reads it through a volatile reference.
constexpr int f() {
  int x = 1;
  volatile int& r = x;
  return r;
}
static_assert(f() == 1);
