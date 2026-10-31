// tidal-rules: CE-23 CX-1
// tidal-expect: reject
// tidal-error-clang: base class .Logged. of non-literal type
// tidal-error-gcc: base class .Logged. of .Derived. is non-literal
// tidal-error-edg: cannot call non-constexpr function
struct Logged {
  ~Logged() {}
};
struct Derived : Logged {
  constexpr Derived() {}
};
constexpr Derived derived;
