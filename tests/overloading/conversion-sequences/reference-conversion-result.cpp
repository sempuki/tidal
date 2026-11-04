// tidal-rules: ICS-11
// tidal-expect: accept
// tidal-note: Both bindings use the same conversion function. The second part to Middle& is the identity and to Base& a derived-to-base step.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Base {};
struct Middle : Base {};
Middle m;
struct Source { operator Middle&() { return m; } };
Source s;
One take(Base&);
Two take(Middle&);
static_assert(same<decltype(take(s)), Two>);
