// tidal-rules: PACK-21
// tidal-expect: accept
// tidal-note: using Bs::f... declares the f of each base, so the two overloads meet in one set. using Bs::Bs... inherits the constructors of each base.
struct Small { constexpr int f(int) const { return 1; } };
struct Text { constexpr int f(char) const { return 2; } };
template <class... Bs> struct Merge : Bs... { using Bs::f...; };
static_assert(Merge<Small, Text>{}.f(0) == 1 && Merge<Small, Text>{}.f('c') == 2);
struct ByInt { constexpr int operator()(int) const { return 1; } };
struct ByDouble { constexpr int operator()(double) const { return 2; } };
template <class... F> struct Overloads : F... { using F::operator()...; };
static_assert(Overloads<ByInt, ByDouble>{}(1) == 1 && Overloads<ByInt, ByDouble>{}(1.0) == 2);
struct FromInt { int v = 0; constexpr FromInt() {} constexpr FromInt(int x) : v(x) {} };
struct FromText { int w = 0; constexpr FromText() {} constexpr FromText(const char*) : w(9) {} };
template <class... Bs> struct Inherit : Bs... { using Bs::Bs...; };
constexpr Inherit<FromInt, FromText> a(3);
constexpr Inherit<FromInt, FromText> b("x");
static_assert(a.v == 3 && b.w == 9);
