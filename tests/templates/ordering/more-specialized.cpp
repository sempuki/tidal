// tidal-rules: ORD-1
// tidal-expect: accept
// tidal-note: Each call has identity sequences for every candidate, so only partial ordering decides. const T* is more specialized than T*, and T* than T.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Tag {};
struct One {}; struct Two {}; struct Three {};
template <class T> One f(T);
template <class T> Two f(T*);
template <class T> Three f(const T*);
int* p = nullptr;
const int* q = nullptr;
static_assert(same<decltype(f(1)), One>);
static_assert(same<decltype(f(p)), Two>);
static_assert(same<decltype(f(q)), Three>);
