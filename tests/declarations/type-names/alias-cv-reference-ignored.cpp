// tidal-rules: TYN-1
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
using R = int&;
using CR = const R;
static_assert(same<CR, int&>);
using F = void();
using CF = const F;
static_assert(same<CF, void()>);
using A = int[2];
using CA = const A;
static_assert(same<CA, const int[2]>);
