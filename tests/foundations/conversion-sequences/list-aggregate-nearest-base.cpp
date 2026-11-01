// tidal-rules: ICS-18 ICS-36
// tidal-expect: accept
// tidal-note: For an aggregate, a list with one Derived element has the element's sequence. ICS-36 prefers the nearer base.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Base { int x; };
struct Middle : Base {};
struct Derived : Middle {};
Derived d;
One take(Middle);
Two take(Base);
static_assert(same<decltype(take({d})), One>);
