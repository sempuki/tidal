// tidal-rules: INST-31
// tidal-expect: run
namespace lib { struct Item {}; }
template <class T> int info(T item) { return probe(item); }
namespace lib { int probe(Item) { return 4; } }
int use() { return info(lib::Item{}); }
int main() { return use() == 4 ? 0 : 1; }
