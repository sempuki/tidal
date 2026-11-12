// tidal-rules: TARG-13 TARG-16
// tidal-expect: reject
// tidal-expect-clang: accept
// tidal-error-gcc: conversion from .int. to .double. in a converted constant expression
// tidal-error-edg: is incompatible with template parameter of type
// tidal-note: RV11-R-7. The braced value 1 is an int, and the parameter is a double. The value converts as without braces, and that conversion is not allowed. GCC and EDG reject. Clang accepts (Q-0454).
template <double D> struct Rate { static constexpr double value = D; };
static_assert(Rate<{1}>::value == 1.0);
