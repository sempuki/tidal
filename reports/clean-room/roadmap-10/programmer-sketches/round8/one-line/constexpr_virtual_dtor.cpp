struct B { constexpr virtual ~B() = default; constexpr virtual int f() const { return 1; } };
struct D : B { constexpr int f() const override { return 2; } };
constexpr int g() { B* p = new D; int r = p->f(); delete p; return r; }
static_assert(g() == 2);
int main() {}
