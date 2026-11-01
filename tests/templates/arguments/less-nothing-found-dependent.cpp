// tidal-rules: TARG-23
// tidal-expect: run
// tidal-note: In the template, ordinary lookup of pick finds nothing, so < opens an argument list. At instantiation, argument-dependent lookup finds the template in lib.
namespace lib {
  struct Tag {};
  template <int N> int pick(Tag) { return N; }
}
template <class T> int call(T t) { return pick<5>(t); }
int main() { return call(lib::Tag{}) == 5 ? 0 : 1; }
