// tidal-rules: CLS-26
// tidal-expect: reject
// tidal-error-clang: cannot be redeclared
// tidal-error-gcc: cannot be overloaded
// tidal-note: Gap review 2026-11-05 of the text phase 10 wrote (reports/review/2026-11-05-gap-review-phase-10-classes-declarations-foundations.md). A member function with no ref-qualifier and one whose explicit object parameter takes the class by value have the same object type once references are removed, so they clash.
struct S {
  void f();
  void f(this S);
};
