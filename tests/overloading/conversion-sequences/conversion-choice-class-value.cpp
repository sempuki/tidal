// tidal-rules: ICS-42
// tidal-expect: accept
// tidal-note: Both results convert to Base by conversion rank, from different classes. Only the nearer source decides.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Base { int x; };
struct Middle : Base {};
struct Derived : Middle {};
struct Source {
  constexpr operator Middle() const { return {{1}}; }
  constexpr operator Derived() const { return {{{2}}}; }
};
constexpr Base b = Source{};
static_assert(b.x == 1);
