// tidal-rules: DEFARG-4
// tidal-expect: reject
// tidal-error-clang: invalid use of .this.
// tidal-error-gcc: this. may not be used in this context
// tidal-error-edg: this. may only be used inside a nonstatic member function
// tidal-note: Gap review 2026-10-30.
struct S {
  int f(const S* p = this);
};
