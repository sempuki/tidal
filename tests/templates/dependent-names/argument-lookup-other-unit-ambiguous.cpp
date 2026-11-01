// tidal-rules: DEP-38
// tidal-expect: run
// tidal-sources: argument-lookup-other-unit-ambiguous.tu.cpp
// tidal-note: The other unit declares probe(Item, unsigned). With it, the call probe(item, 0) would be ambiguous. This unit never sees it. DEP-38 needs no diagnostic, and the program accepts the result that this unit gives.
namespace lib { struct Item {}; int probe(Item, long) { return 1; } }
template <class T> int info(T item) { return probe(item, 0); }
int main() { return info(lib::Item{}) == 1 ? 0 : 1; }
