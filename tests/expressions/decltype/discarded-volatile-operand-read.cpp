// tidal-rules: UNEV-2
// tidal-expect: reject
// tidal-error-gcc: conversion of a volatile lvalue
// tidal-note: Gap review 2026-10-31.
constexpr int f() { volatile int v = 1; v + 0; return 0; }
static_assert(f() == 0);
