// tidal-rules: OVL-36 OVL-14
// tidal-expect: reject
// tidal-error-clang: conversion from .Source. to .Target. is ambiguous
// tidal-error-gcc: conversion from .Source. to .Target. is ambiguous
// tidal-error-edg: more than one user-defined conversion from .Source. to .Target. applies
// tidal-note: The constructor and the conversion function both bind const Source& to the lvalue. Their sequences tie.
struct Source;
struct Target { Target(const Source&); };
struct Source { operator Target() const; };
Source s;
Target t = s;
