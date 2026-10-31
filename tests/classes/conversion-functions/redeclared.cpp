// tidal-rules: CVF-7
// tidal-expect: reject
// tidal-error-clang: conversion function cannot be redeclared
// tidal-error-gcc: cannot be overloaded with
// tidal-error-edg: invalid redeclaration of member function
struct S { operator int(); operator int(); };
