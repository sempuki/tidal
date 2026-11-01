// tidal-rules: INST-32 INST-31
// tidal-expect: run
// tidal-note: Right after use, probe is not declared. At the end of the unit, probe is declared. The two points differ, so INST-32 allows any result. Every compiler finds probe.
namespace lib { struct Item {}; }
template <class T> int info(T item) { return probe(item); }
int use() { return info(lib::Item{}); }
namespace lib { int probe(Item) { return 4; } }
int main() { return use() == 4 ? 0 : 1; }
