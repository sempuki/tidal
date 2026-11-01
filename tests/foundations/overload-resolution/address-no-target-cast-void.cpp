// tidal-rules: OVL-20
// tidal-expect: reject
// tidal-error-clang: cannot be cast to type .void.
// tidal-error-gcc: conversion to void cannot resolve address of overloaded function
// tidal-error-edg: overloaded|cannot|no instance|deduce
// tidal-note: A cast to void names no function type, so it gives no target type.
void f(int);
void f(double);
void use() { (void)f; }
