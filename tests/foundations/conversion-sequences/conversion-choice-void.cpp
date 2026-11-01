// tidal-rules: ICS-43
// tidal-expect: accept
// tidal-note: Both results convert to void* by conversion rank. Only the base-to-void preference decides.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Base {};
struct Derived : Base {};
Base b;
struct Source { constexpr operator Derived*() const { return nullptr; } constexpr operator Base*() const { return &b; } };
constexpr void* p = Source{};
static_assert(p == &b);
