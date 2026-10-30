// tidal-rules: CV-1
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
static_assert(same<const int, int const>);
static_assert(same<const int*, int const*>);
int x;
const int* a = &x;
int* const b = &x;
const int* const c = &x;
static_assert(same<decltype(a), const int*>);
static_assert(same<decltype(b), int* const>);
static_assert(same<decltype(c), const int* const>);
static_assert(!same<const int*, int* const>);
