// tidal-rules: NAM-13
// tidal-expect: accept
namespace inner { long value = 4; }
namespace outer { int value = 3; using namespace inner; }
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
static_assert(same<decltype(outer::value), int>);
