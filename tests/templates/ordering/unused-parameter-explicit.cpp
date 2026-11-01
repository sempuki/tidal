// tidal-rules: ORD-3
// tidal-expect: accept
// tidal-note: R is given explicitly and appears in no function parameter type. It needs no value when one template is deduced from the other, so f(T*) is more specialized.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct One {}; struct Two {};
template <class R, class T> One f(T);
template <class R, class T> Two f(T*);
int* p = nullptr;
static_assert(same<decltype(f<long>(p)), Two>);
