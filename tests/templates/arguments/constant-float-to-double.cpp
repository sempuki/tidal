// tidal-rules: TARG-13
// tidal-expect: reject
// tidal-error-clang: conversion from 'float' to 'double' is not allowed in a converted constant expression
// tidal-error-gcc: conversion from .float. to .double. in a converted constant expression
// tidal-error-edg: incompatible
// tidal-note: A floating-point promotion is not one of the conversions a constant template argument allows, so 1.5f cannot be the argument for a double parameter.
template <double D> struct Scale { };
Scale<1.5f> half;
