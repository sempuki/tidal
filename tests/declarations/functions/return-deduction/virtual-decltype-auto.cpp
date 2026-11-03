// tidal-rules: RET-5
// tidal-expect: reject
// tidal-error-clang: deduced return type cannot be virtual
// tidal-error-gcc: virtual function cannot have deduced return type
// tidal-error-edg: virtual function cannot have a deduced return type
// tidal-note: The virtual function uses decltype(auto).
struct S {
  int v = 1;
  virtual decltype(auto) get() { return v; }
};
