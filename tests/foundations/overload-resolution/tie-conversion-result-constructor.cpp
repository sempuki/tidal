// tidal-rules: OVL-26
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-error-clang: conversion from .const Source. to .const Target. is ambiguous
// tidal-note: Q-0189. The constructor and the conversion function both bind const Source& to s, so the argument sequences tie. The constructor's result is a Target, and the conversion function's result needs a derived-to-base conversion. OVL-26 prefers the constructor. Clang finds the two ambiguous.
struct Source;
struct Target {
  int from;
  constexpr Target(const Source&) : from(1) {}
  constexpr Target(int v) : from(v) {}
};
struct Derived : Target { constexpr Derived() : Target(2) {} };
struct Source { constexpr operator Derived() const { return Derived(); } };
constexpr Source s;
constexpr Target t = s;
static_assert(t.from == 1);
