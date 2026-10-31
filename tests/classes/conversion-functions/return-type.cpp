// tidal-rules: CVF-7
// tidal-expect: reject
// tidal-error-clang: conversion function cannot have a return type
// tidal-error-gcc: return type specified for .operator int
// tidal-error-edg: return type may not be specified on a conversion function
struct S { int operator int(); };
