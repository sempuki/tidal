// tidal-rules: PRIM-19
// tidal-expect: reject
// tidal-error-clang: conversion from string literal to .char \*.
// tidal-error-gcc: converting a string constant to .char\*.
// tidal-error-edg: const char \*. cannot be used to initialize an entity of type .char \*
char* p = "abc";
