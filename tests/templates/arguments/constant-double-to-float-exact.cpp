// tidal-rules: TARG-13
// tidal-expect: reject
// tidal-error-clang: conversion from 'double' to 'float' is not allowed in a converted constant expression
// tidal-error-gcc: conversion from .double. to .float. in a converted constant expression
// tidal-error-edg: incompatible
// tidal-note: 1.5 is exact in float, and the conversion from double to float is still not allowed for a constant template argument.
template <float F> struct Scale { };
Scale<1.5> half;
