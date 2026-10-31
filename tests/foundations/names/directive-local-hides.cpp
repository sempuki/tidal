// tidal-rules: NAM-23
// tidal-expect: accept
namespace m { long value = 1; }
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
void test() {
  int value = 2;
  {
    using namespace m;
    static_assert(same<decltype(value), int>);
  }
}
