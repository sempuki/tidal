// tidal-rules: OVL-20
// tidal-expect: accept
// tidal-note: In each position with no target type, a set of one plain function, or a template-id that fixes every template parameter, gives that function.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
void once(int);
template <class T> void generic(T);
auto a = once;
auto b = generic<int>;
auto get() { return once; }
auto c = [h = once] { return h; };
template <auto P> struct Hold {};
Hold<once> hold;
void use() { (void)once; once; }
static_assert(same<decltype(a), void (*)(int)>);
static_assert(same<decltype(b), void (*)(int)>);
static_assert(same<decltype(get()), void (*)(int)>);
