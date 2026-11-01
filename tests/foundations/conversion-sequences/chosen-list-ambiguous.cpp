// tidal-rules: ICS-8 CTOR-25
// tidal-expect: reject
// tidal-error-clang: call to constructor of .Split. is ambiguous
// tidal-error-gcc: conversion from .<brace-enclosed initializer list>. to .Split. is ambiguous
// tidal-error-edg: more than one instance of constructor .Split::Split.
// tidal-note: No constructor of Split is best for {1, 1}, so the sequence is the ambiguous sequence (ICS-19). It counts as user-defined and beats the ellipsis, so take(Split) is chosen and the list fails.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Split { Split(int, long); Split(long, int); };
One take(Split);
Two take(...);
auto r = take({1, 1});
