// tidal-rules: ICS-8 ICS-7 CVF-12
// tidal-expect: reject
// tidal-error-clang: conversion from .Source. to .Target. is ambiguous
// tidal-error-gcc: conversion from .Source. to .Target. is ambiguous
// tidal-error-edg: more than one user-defined conversion from .Source. to .Target.
// tidal-note: Target(Source&) and operator Target() both bind s to Source& with the identity, so they tie and the sequence is the ambiguous sequence. It beats the ellipsis, so take(Target) is chosen.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Target;
struct Source { operator Target(); };
struct Target { Target(); Target(Source&); };
Source s;
One take(Target);
Two take(...);
auto r = take(s);
