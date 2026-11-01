// tidal-rules: INST-32
// tidal-expect: run
// tidal-sources: points-in-two-units.tu.cpp
// tidal-note: info<lib::Item> calls probe(Item, double) in this unit and probe(Item, int) in the other. INST-32 allows any result, and the program accepts each pair. The -O0 builds use one definition for both calls, the -O2 builds inline each.
namespace lib { struct Item {}; inline int probe(Item, double) { return 1; } }
template <class T> int info(T item) { return probe(item, 0); }
int here() { return info(lib::Item{}); }
int there();
int main() {
  int a = here(), b = there();
  return (a == 1 || a == 2) && (b == 1 || b == 2) ? 0 : 1;
}
