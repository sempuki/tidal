// tidal-rules: DEP-1 DEP-9
// tidal-expect: run
namespace shop {
template <int N> struct Tag {};
template <class... T> struct List {};
}
template <int N> int by_value() { shop::Tag<N> t; return price(t); }
template <class... T> int by_pack() { shop::List<T...> l; return price(l); }
namespace shop {
template <int N> int price(Tag<N>) { return N; }
template <class... T> int price(List<T...>) { return 1; }
}
int main() { return by_value<7>() == 7 && by_pack<>() == 1 ? 0 : 1; }
