// tidal-rules: TSP-15 ORD-14
// tidal-expect: run
// tidal-note: Roadmap round 10, programmer reduction partial_spec_array_vs_const_array.cpp. Both partial specializations match const char[4]. By ORD-14, the one for const T[N] is more specialized than the one for T[N], so TSP-15 selects it.
namespace std { using size_t = decltype(sizeof 0); }
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Decay { using type = T; };
template <class T, std::size_t N> struct Decay<T[N]> { using type = T*; };
template <class T, std::size_t N> struct Decay<const T[N]> { using type = const T*; };
static_assert(same<Decay<const char[4]>::type, const char*>);
static_assert(same<Decay<int[2]>::type, int*>);
int main() {
  Decay<const char[4]>::type p = "abc";
  return p[2] == 'c' ? 0 : 1;
}
