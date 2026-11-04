// tidal-rules: COND-16 COND-5
// tidal-expect: reject
// tidal-error-clang: conversion from 'Wide' to 'short' is ambiguous
// tidal-error-gcc: conversion from .Wide. to .short int. is ambiguous
// tidal-error-edg: more than one user-defined conversion from "Wide" to "short"
// tidal-note: The conversions of Wide to short through int and through long are equally good, so the conversion sequence is ambiguous.
struct Wide { operator int(); operator long(); };
extern bool c;
Wide w;
short s;
auto x = c ? w : s;
