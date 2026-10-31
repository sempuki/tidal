// tidal-rules: TYN-1
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
using CI = const int;
using P = CI*;
static_assert(same<P, const int*>);
using PC = int* const;
using PCC = const PC;
static_assert(same<PCC, int* const>);
