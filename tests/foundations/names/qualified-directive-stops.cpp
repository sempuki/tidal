// tidal-rules: NAM-13
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
namespace deep { long value = 1; }
namespace mid { int value = 2; using namespace deep; }
namespace top { using namespace mid; }
static_assert(same<decltype(top::value), int>);
