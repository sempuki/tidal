// tidal-rules: TARG-23
// tidal-expect: run
// tidal-note: Ordinary lookup of pick finds nothing in the first call and a plain function in the second. In both, < opens an argument list, and argument-dependent lookup finds the template in lib.
namespace lib {
  struct Tag {};
  template <int N> int pick(Tag) { return N; }
}
int first() { return pick<3>(lib::Tag{}); }
int pick(int, int) { return 0; }
int second() { return pick<4>(lib::Tag{}); }
int main() { return first() == 3 && second() == 4 ? 0 : 1; }
