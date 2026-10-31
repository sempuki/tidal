// tidal-rules: VIRT-6
// tidal-expect: accept
struct B {
  int seen;
  constexpr B() : seen(f()) {}
  constexpr virtual int f() { return 1; }
};
struct D : B { constexpr int f() override { return 2; } };
constexpr D d;
static_assert(d.seen == 1);
