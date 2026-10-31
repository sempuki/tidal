// tidal-rules: NAM-24 NAM-25
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
namespace outer {
  double pick(double);
  namespace lib { int pick(int); }
  namespace user {
    using namespace lib;
    static_assert(same<decltype(pick(1)), int>);
    static_assert(same<decltype(pick(1.0)), double>);
  }
}
