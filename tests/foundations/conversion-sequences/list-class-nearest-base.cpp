// tidal-rules: ICS-19 ICS-36
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-error-edg: more than one instance of overloaded function
// tidal-note: Q-0154. A list with one Derived element has the element's sequence, a derived-to-base conversion. ICS-36 prefers the nearer base. EDG finds a tie.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Base { Base(); };
struct Middle : Base { Middle(); };
struct Derived : Middle { Derived(); };
Derived d;
One take(Middle);
Two take(Base);
static_assert(same<decltype(take({d})), One>);
