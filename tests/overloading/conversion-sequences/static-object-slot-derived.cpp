// tidal-rules: ICS-14
// tidal-expect: accept
// tidal-note: The object is of a derived class, so the object slot of the member that is not static needs a derived-to-base conversion. The slot of the static member ties with that conversion, the first arguments tie, and the second argument decides.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Tool { static One use(long, int); Two use(long, long) const; };
struct Tool2 { static One use(long, long); Two use(long, int) const; };
struct Derived : Tool {};
struct Derived2 : Tool2 {};
Derived d;
Derived2 d2;
static_assert(same<decltype(d.use(1, 0)), One>);
static_assert(same<decltype(d2.use(1, 0)), Two>);
