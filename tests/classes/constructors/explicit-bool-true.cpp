// tidal-rules: CTOR-9
// tidal-expect: reject
// tidal-error-clang: no viable conversion from .int. to .S
// tidal-error-gcc: conversion from .int. to non-scalar type .S
// tidal-error-edg: no suitable constructor exists to convert from .int. to .S
struct S { explicit(true) S(int) {} };
S s = 1;
