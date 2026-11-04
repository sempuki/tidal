// tidal-rules: ICS-19
// tidal-expect: accept
// tidal-note: One element of the class gives an exact-match sequence, and of a derived class a conversion-rank sequence, both standard.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Cell { Cell(); Cell(int); };
struct Wrap { Wrap(Cell); };
struct Derived : Cell {};
struct Holder { Holder(const Derived&); };
Cell c;
Derived d;
One take(Cell);
Two take(Wrap);
static_assert(same<decltype(take({c})), One>);
One base(Cell);
Two base(Holder);
static_assert(same<decltype(base({d})), One>);
One rank(Cell);
Two rank(long);
static_assert(same<decltype(rank({d})), One>);
