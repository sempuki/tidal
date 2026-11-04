// tidal-rules: ICS-47
// tidal-expect: accept
// tidal-note: Each g is a candidate on its own. A function parameter selects from the set by its type, and any other parameter gets the one function of a set that needs no target, or is not viable.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
void twice(int);
void twice(double);
void once(int);
void exact(int) noexcept;
void exact(double);
template <class T> void generic(T);

namespace a { One g(bool); Two g(void (*)(int)); }
static_assert(same<decltype(a::g(twice)), Two>);     // g(bool) is not viable
static_assert(same<decltype(a::g(once)), Two>);      // identity beats bool

namespace b { One g(bool); Two g(...); }
static_assert(same<decltype(b::g(once)), One>);      // a set of one acts as the function

namespace c { One g(void (*)(char)); Two g(...); }
static_assert(same<decltype(c::g(once)), Two>);      // no match for char
static_assert(same<decltype(c::g(generic<int>)), Two>);

namespace d { One g(void (*)(int)); Two g(void (*)(int) noexcept); }
static_assert(same<decltype(d::g(exact)), Two>);     // no conversion beats dropping noexcept

namespace e { One g(void (*)(int)); Two g(...); }
static_assert(same<decltype(e::g(twice)), One>);
