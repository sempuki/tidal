// tidal-rules: CTOR-22
// tidal-expect: reject
// tidal-error-clang: no viable conversion from .const char\[4\]. to .Person.
// tidal-error-gcc: conversion from .const char \[4\]. to non-scalar type .Person. requested
// tidal-error-edg: no suitable constructor exists to convert from .const char \[4\]. to .Person.
// tidal-note: Copy-initialization from a string literal would need the constructor of Name and then the constructor of Person.
struct Name { Name(const char*) {} };
struct Person { Person(Name) {} };
Person p = "Ada";
