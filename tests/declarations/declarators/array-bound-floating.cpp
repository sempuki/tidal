// tidal-rules: DCL-5
// tidal-expect: reject
// tidal-error-clang: conversion from .double. to .* is not allowed in a converted constant expression
// tidal-error-gcc: conversion from .double. to .long unsigned int. in a converted constant expression
// tidal-error-edg: has type .double. instead of the required
int a[2.0];
