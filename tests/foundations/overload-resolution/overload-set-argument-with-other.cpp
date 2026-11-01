// tidal-rules: OVL-39
// tidal-expect: accept
// tidal-note: A call by name, with an overload set and an int as arguments. Only the pairing of f(int) with the first g matches the set, and that g also takes the int exactly.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
void f(int);
void f(double);
One g(void (*)(int), int);
Two g(void (*)(char), long);
struct Hold {
  static One h(void (*)(int));
  static Two h(void (*)(char));
};
static_assert(same<decltype(g(f, 1)), One>);
static_assert(same<decltype(Hold::h(f)), One>);
static_assert(same<decltype(Hold{}.h(f)), One>);
