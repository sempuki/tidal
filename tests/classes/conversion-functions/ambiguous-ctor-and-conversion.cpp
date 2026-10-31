// tidal-rules: CVF-4
// tidal-expect: reject
// tidal-error-clang: conversion from .A. to .B. is ambiguous
// tidal-error-gcc: conversion from .A. to .B. is ambiguous
// tidal-error-edg: more than one user-defined conversion
struct B;
struct A { operator B() const; };
struct B { B() {} B(const A&) {} };
void take(B);
void f() { take(A{}); }
