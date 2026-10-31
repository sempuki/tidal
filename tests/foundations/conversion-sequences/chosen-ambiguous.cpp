// tidal-rules: ICS-8 ICS-7 CVF-12
// tidal-expect: reject
// tidal-error-clang: conversion from .Source. to .double. is ambiguous
// tidal-error-gcc: conversion from .Source. to .double. is ambiguous
// tidal-error-edg: more than one conversion function from .Source. to .double.
// tidal-note: The ambiguous sequence beats the ellipsis (ICS-21), so take(double) is chosen and its conversion fails.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Source { operator int(); operator long(); };
One take(double);
Two take(...);
auto r = take(Source{});
