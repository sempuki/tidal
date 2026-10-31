// tidal-rules: CVF-7
// tidal-expect: reject
// tidal-error-clang: conversion function must be a non-static member
// tidal-error-edg: conversion function must be a nonstatic member
struct S {};
operator int(S);
