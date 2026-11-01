// tidal-rules: INST-32
// tidal-expect: run
// tidal-note: Right after use, only probe(Item, double) is declared. At the end of the unit, probe(Item, int) is a better match. INST-32 allows either, and the program accepts both results.
namespace lib { struct Item {}; int probe(Item, double) { return 1; } }
template <class T> int info(T item) { return probe(item, 0); }
int use() { return info(lib::Item{}); }
namespace lib { int probe(Item, int) { return 2; } }
int main() { int r = use(); return r == 1 || r == 2 ? 0 : 1; }
