// tidal-rules: CVF-4
// tidal-expect: reject
// tidal-error-clang: conversion from .S. to .double. is ambiguous
// tidal-error-gcc: conversion from .S. to .double. is ambiguous
// tidal-error-edg: more than one conversion function
struct S { operator int() const { return 1; } operator long() const { return 2; } };
double d = S{};
