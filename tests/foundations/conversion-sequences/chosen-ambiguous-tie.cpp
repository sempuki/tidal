// tidal-rules: ICS-8 ICS-7 CVF-12
// tidal-expect: reject
// tidal-error-clang: conversion from .Source. to .double. is ambiguous
// tidal-error-gcc: conversion from .Source. to .double. is ambiguous
// tidal-error-edg: more than one conversion function from .Source. to .double.
// tidal-note: The first arguments tie and the second decides for take(double, int), whose first conversion is ambiguous.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Source { operator int(); operator long(); };
struct Wrap { Wrap(Source); };
One take(double, int);
Two take(Wrap, long);
auto r = take(Source{}, 0);
