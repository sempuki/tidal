// tidal-rules: OVL-14 ICS-10 OVL-35
// tidal-expect: reject
// tidal-error-clang: no viable conversion from .ToMid. to .Target.
// tidal-error-gcc: conversion from .ToMid. to non-scalar type .Target. requested
// tidal-error-edg: no suitable user-defined conversion from .ToMid. to .Target.
// tidal-note: In copy initialization from another class, Target(Mid) would need ToMid to reach Mid through a conversion function. A constructor gets the initializer by a standard sequence only, so no candidate is viable. Direct initialization of the same kind is valid.
struct Mid {};
struct Target { Target(Mid); };
struct ToMid { operator Mid() const; };
Target direct(ToMid{});
Target copied = ToMid{};
