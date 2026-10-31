// tidal-rules: VIRT-12
// tidal-expect: reject
// tidal-error-gcc: __cxa_pure_virtual
struct B {
  int v;
  constexpr B() : v(call()) {}
  constexpr int call() { return f(); }
  constexpr virtual int f() = 0;
};
struct D : B { constexpr int f() override { return 2; } };
constexpr D d;
