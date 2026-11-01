// tidal-rules: DEP-38
// tidal-expect: run
// tidal-sources: argument-lookup-other-unit.tu.cpp
// tidal-note: The other unit declares probe(Item, int), a better match that this unit never sees. DEP-38 needs no diagnostic, and the program accepts either result.
namespace lib { struct Item {}; int probe(Item, double) { return 1; } }
template <class T> int info(T item) { return probe(item, 0); }
int main() { int r = info(lib::Item{}); return r == 1 || r == 2 ? 0 : 1; }
