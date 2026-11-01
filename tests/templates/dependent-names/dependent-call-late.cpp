// tidal-rules: DEP-2 DEP-8
// tidal-expect: run
namespace lib {
struct Tag {};
int probe(Tag, double) { return 1; }
}
template <class T> int use() { return probe(lib::Tag{}, T(0)); }
namespace lib {
int probe(Tag, int) { return 2; }
}
int main() { return use<int>() == 2 ? 0 : 1; }
