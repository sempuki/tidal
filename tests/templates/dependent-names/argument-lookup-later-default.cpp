// tidal-rules: DEP-9
// tidal-expect: run
namespace lib { struct Item {}; int probe(Item, int); }
template <class T> int info(T item) { return probe(item); }
namespace lib { int probe(Item, int = 4); }
int main() { return info(lib::Item{}) == 4 ? 0 : 1; }
namespace lib { int probe(Item, int n) { return n; } }
