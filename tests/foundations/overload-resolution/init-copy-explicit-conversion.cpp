// tidal-rules: OVL-14 OVL-35
// tidal-expect: reject
// tidal-error-clang: no viable conversion from .Source. to .Target.
// tidal-error-gcc: conversion from .Source. to non-scalar type .Target. requested
// tidal-error-edg: no suitable user-defined conversion from .Source. to .Target. exists
// tidal-note: The same classes as init-direct-explicit-conversion.cpp. Copy-initialization takes only conversion functions that are not explicit.
struct Target { Target(int); };
struct Source { explicit operator Target() const; };
Source s;
Target t = s;
