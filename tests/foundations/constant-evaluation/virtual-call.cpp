// tidal-rules: CE-1
// tidal-expect: accept
struct Base { constexpr virtual int id() const { return 1; } };
struct Derived : Base { constexpr int id() const override { return 2; } };
constexpr int call(const Base& b) { return b.id(); }
constexpr Derived d;
static_assert(call(d) == 2);
