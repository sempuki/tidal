// tidal-rules: CV-1
// tidal-expect: accept
// tidal-note: A const before a type alias for a pointer applies to the pointer. Each cv-qualifier in a pointer to pointer applies to the level on its left.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
using P = int*;
static_assert(same<const P, int* const>);
static_assert(same<P const, int* const>);
int x;
int* px = &x;
int* const* volatile pp = &px;
static_assert(same<decltype(pp), int* const* volatile>);
static_assert(same<decltype(*pp), int* const&>);
