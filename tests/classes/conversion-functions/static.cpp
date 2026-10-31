// tidal-rules: CVF-7
// tidal-expect: reject
// tidal-error-clang: conversion function must be a non-static member
// tidal-error-gcc: must be a non-static member function
// tidal-error-edg: conversion function must be a nonstatic member
struct S { static operator int(); };
