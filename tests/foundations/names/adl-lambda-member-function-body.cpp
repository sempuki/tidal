// tidal-rules: NAM-28
// tidal-expect: reject
// tidal-error-clang: use of undeclared identifier .take.
// tidal-error-gcc: .take. was not declared in this scope
// tidal-error-edg: identifier .take. is undefined
// tidal-note: The lambda is in the body of a member function, so its closure type belongs to the block. The class C is not associated, and its friend take is not found.
struct C {
  template <class F> friend int take(F) { return 0; }
  int m() { return take([] {}); }
};
