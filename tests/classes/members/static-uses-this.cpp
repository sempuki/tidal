// tidal-rules: CLS-7
// tidal-expect: reject
// tidal-error-clang: invalid use of .this. outside of a non-static member function
// tidal-error-gcc: this. is unavailable for static member functions
// tidal-error-edg: this. may only be used inside a nonstatic member function
struct S {
  int v;
  static int get() { return this->v; }
};
