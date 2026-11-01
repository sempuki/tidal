// tidal-rules: LOG-3
// tidal-expect: reject
// tidal-error-clang: conversion from .Both. to .bool. is ambiguous
// tidal-error-gcc: conversion from .Both. to .bool. is ambiguous
// tidal-error-edg: more than one conversion function from .Both.
struct Both {
  operator int() const { return 1; }
  operator double() const { return 1.0; }
};
bool b = Both{} && true;
