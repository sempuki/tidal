// tidal-rules: CVF-7
// tidal-expect: reject
// tidal-error-clang: conversion function cannot have any parameters
// tidal-error-gcc: must have no arguments
// tidal-error-edg: no arguments allowed on user-defined conversion
struct S { operator int(int); };
