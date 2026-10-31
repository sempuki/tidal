// tidal-rules: NAM-25
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
namespace a { int pick(int); }
namespace b { double pick(double); }
using namespace a;
using namespace b;
static_assert(same<decltype(pick(1)), int>);
static_assert(same<decltype(pick(1.0)), double>);
