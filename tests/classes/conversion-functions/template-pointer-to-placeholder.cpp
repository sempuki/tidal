// tidal-rules: CVF-7
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-expect-gcc-14: accept
// tidal-error-clang: not allowed in declaration of conversion function template
// tidal-error-gcc: invalid use of .auto. in conversion operator
// tidal-note: Gap review 2026-11-05 (reports/review/2026-11-05-gap-review-phase-9-classes-overloading-exceptions.md). A conversion function template cannot deduce its target type, also when the target is a pointer to a placeholder. Clang and GCC 16 reject. EDG and GCC 14 accept.
struct S {
  int v = 1;
  template <class T> operator auto*() { return &v; }
};
