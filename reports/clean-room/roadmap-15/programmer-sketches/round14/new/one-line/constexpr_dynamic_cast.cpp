struct B { virtual ~B() = default; };
struct D : B { int v = 7; };
struct E : B {};
constexpr int probe() { D d; E e; B* p = &d; B* q = &e; return (dynamic_cast<D*>(p) ? dynamic_cast<D*>(p)->v : 0) + (dynamic_cast<D*>(q) ? 100 : 0); }
static_assert(probe() == 7);
int main() { return probe() - 7; }
