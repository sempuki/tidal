// tidal-rules: CLS-5
// tidal-expect: reject
// tidal-error-clang: within const member function
// tidal-error-gcc: in read-only object
// tidal-error-edg: must be a modifiable lvalue
struct S {
  int v;
  void set() const { v = 1; }
};
