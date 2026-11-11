// tidal-rules: CTAD-17 CTAD-5
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: conversion from .char. to non-scalar type
// tidal-note: For 'c', the guide from N(T) deduces T = char and is explicit for char, so copy initialization leaves it out before the choice. The user guide N(int) -> N<int> is then chosen. GCC chooses the guide from N(T) and then rejects the copy initialization of N<char> (Q-0437). Direct initialization takes the better guide.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct N { constexpr explicit(sizeof(T) == 1) N(T) {} };
N(int) -> N<int>;
N copied = 'c';
N direct('c');
static_assert(same<decltype(copied), N<int>> && same<decltype(direct), N<char>>);
