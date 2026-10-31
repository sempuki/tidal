// tidal-rules: ICS-33 ICS-36
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: call of overloaded .(take|near).* is ambiguous
// tidal-note: Q-0137. The array-to-pointer step is not compared (ICS-26), so ICS-33 and ICS-36 decide as for a pointer. GCC finds both calls ambiguous.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Base {};
struct Middle : Base {};
struct Derived : Middle {};
Derived items[2];
One take(void*);
Two take(Base*);
static_assert(same<decltype(take(items)), Two>);
One near(Base*);
Two near(Middle*);
static_assert(same<decltype(near(items)), Two>);
