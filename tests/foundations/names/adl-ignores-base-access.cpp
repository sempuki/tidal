// tidal-rules: NAM-30 ACC-12
// tidal-expect: reject
// tidal-error-clang: cannot cast .B. to its private base class
// tidal-error-gcc: lib::A. is an inaccessible base of .B
// tidal-error-edg: conversion to inaccessible base class .lib::A. is not allowed
namespace lib { struct A { }; int probe(const A*) { return 1; } }
struct B : private lib::A { };
int read() { B b; return probe(&b); }
