// tidal-rules: DER-20
// tidal-expect: reject
// tidal-error-clang: no viable conversion from .int. to .D
// tidal-error-gcc: conversion from .int. to non-scalar type .D
// tidal-error-edg: no suitable constructor exists to convert from .int. to .D
struct B { explicit B(int) {} };
struct D : B { using B::B; };
D d = 1;
