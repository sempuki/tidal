// tidal-rules: CVF-7
// tidal-expect: reject
// tidal-error-clang: conversion function cannot be variadic
// tidal-error-gcc: must not have variable number of arguments
// tidal-error-edg: no arguments allowed on user-defined conversion
struct S { operator int(...); };
