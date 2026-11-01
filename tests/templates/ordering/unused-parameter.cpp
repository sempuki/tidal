// tidal-rules: ORD-3
// tidal-expect: accept
// tidal-note: U appears in no compared type, so it needs no value when f(T, U) is deduced from f(T*). f(T*) is more specialized.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Tag {};
struct One {}; struct Two {};
template <class T, class U = void> One f(T);
template <class T> Two f(T*);
int* p = nullptr;
static_assert(same<decltype(f(p)), Two>);
