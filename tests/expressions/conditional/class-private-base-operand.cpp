// tidal-rules: ACC-12 COND-16
// tidal-expect: reject
// tidal-error-clang: cannot cast .Hidden. to its private base class
// tidal-error-gcc: is an inaccessible base of
// tidal-error-edg: conversion to inaccessible base class
// tidal-note: Roadmap round 9 (I4-7), I4 reduction r5-private-base.cpp. The class Hidden derives privately from Root. A Hidden lvalue meets a Root lvalue in main, where the base is not accessible. The conversion counts for COND-16 and then cannot be made.
struct Root {};
struct Hidden : private Root {};
int main() {
  Hidden h;
  Root r;
  (void)(true ? h : r);
  return 0;
}
