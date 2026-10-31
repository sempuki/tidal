// tidal-rules: ICS-11
// tidal-expect: accept
// tidal-note: Binding Base& to a Derived has conversion rank. Copying Derived by value has exact-match rank.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Base {};
struct Derived : Base {};
Derived d;
One take(const Base&);
Two take(Derived);
static_assert(same<decltype(take(d)), Two>);
