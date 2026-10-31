// tidal-rules: VIRT-1
// tidal-expect: accept
struct B { constexpr virtual int f() const { return 1; } };
struct D : B { constexpr int f() const override { return 2; } };
constexpr int call(const B& b) { return b.f(); }
constexpr D d;
static_assert(call(d) == 2);
