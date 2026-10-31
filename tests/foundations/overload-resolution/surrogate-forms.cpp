// tidal-rules: OVL-6
// tidal-expect: accept
// tidal-note: A pointer to function, a reference to such a pointer, and a reference to function each give a surrogate. Each call has one viable surrogate.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
using ToOne = One (*)(int);
using ToTwo = Two (*)(int, int);
using Three = long (&)(int*);
struct Thing {
  operator ToOne() const;
  operator ToTwo&() const;
  operator Three() const;
};
Thing t;
int n = 0;
static_assert(same<decltype(t(1)), One>);
static_assert(same<decltype(t(1, 2)), Two>);
static_assert(same<decltype(t(&n)), long>);
