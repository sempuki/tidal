// tidal-rules: ICS-18
// tidal-expect: accept
// tidal-note: An aggregate from its members is user-defined, so the standard sequence to long wins. One element of the class itself gives that element's exact sequence.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Point { int x; };
struct Derived : Point {};
struct Wrap { Wrap(Point); };
One take(Point);
Two take(long);
static_assert(same<decltype(take({1})), Two>);
Point p{1};
Derived d{};
One copy(Point);
Two copy(Wrap);
static_assert(same<decltype(copy({p})), One>);
One base(Point);
Two base(...);
static_assert(same<decltype(base({d})), One>);
