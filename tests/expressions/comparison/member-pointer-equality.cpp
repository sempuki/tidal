// tidal-rules: CMP-22 CMP-19
// tidal-expect: accept
struct Base { int x; int y; };
struct Derived : Base { int z; };
static_assert(&Base::x == &Base::x);
static_assert(&Base::x != &Base::y);
constexpr int Derived::* px = &Base::x;
static_assert(px == &Derived::x);
static_assert(px != &Derived::z);
constexpr int Base::* none = nullptr;
static_assert(none != &Base::x);
static_assert(none == nullptr);
