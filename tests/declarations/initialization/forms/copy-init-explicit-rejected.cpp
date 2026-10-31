// tidal-rules: INIT-4
// tidal-expect: reject
// tidal-error-clang: no viable conversion from .int. to .S
// tidal-error-gcc: conversion from .int. to non-scalar type .S. requested
// tidal-error-edg: no suitable constructor exists to convert from .int. to .S
struct S { explicit S(int) {} };
S a = 1;
