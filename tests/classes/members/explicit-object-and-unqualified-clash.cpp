// tidal-rules: CLS-26
// tidal-expect: reject
// tidal-error-clang: cannot be redeclared
// tidal-error-gcc: cannot be overloaded
// tidal-note: Gap review 2026-11-05 (reports/review/2026-11-05-gap-review-phase-9-classes-overloading-exceptions.md). A member function with no ref-qualifier and one with an explicit object parameter of type S& have the same parameters and the same object type, so they cannot be overloaded.
struct S {
  void f();
  void f(this S&);
};
