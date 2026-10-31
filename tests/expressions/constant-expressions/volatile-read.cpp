// tidal-rules: CE-12
// tidal-expect: reject
// tidal-error-gcc: conversion of a volatile lvalue
constexpr int f() {
  volatile int v = 1;
  return v;
}
static_assert(f() == 1);
