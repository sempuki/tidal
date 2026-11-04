// tidal-rules: ICS-2
// tidal-expect: accept
// tidal-note: Derived to Base by value has conversion rank. Cell to Cell by value is exact, so it ties with the reference binding and the second argument decides (overload resolution referenced).
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Base { Base(); };
struct Derived : Base {};
Derived d;
One take(Base);
Two take(const Derived&);
static_assert(same<decltype(take(d)), Two>);
struct Cell { Cell(); Cell(const Cell&); };
namespace first { One put(Cell, int); Two put(const Cell&, long); }
namespace second { One put(Cell, long); Two put(const Cell&, int); }
Cell c;
static_assert(same<decltype(first::put(c, 0)), One>);
static_assert(same<decltype(second::put(c, 0)), Two>);
