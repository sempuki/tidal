// tidal-rules: CLS-14
// tidal-expect: reject
// tidal-error-clang: invalid use of .this. in a function with an explicit object parameter
// tidal-error-gcc: this. is unavailable for explicit object member functions
// tidal-error-edg: this. may only be used inside a nonstatic member function
// tidal-note: Gap review 2026-10-30.
struct S {
  int v;
  int get(this S self) { return this->v; }
};
