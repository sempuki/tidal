// tidal-rules: CTOR-12
// tidal-expect: reject
// tidal-error-gcc: accessing uninitialized member .S::b
// tidal-error-clang: read of object outside its lifetime
// tidal-error-edg: access to uninitialized object
struct S {
  int a;
  int b;
  constexpr S() : a(b), b(1) {}
};
constexpr S s;
