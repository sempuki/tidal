// tidal-rules: DEP-3 DEP-5
// tidal-expect: run
namespace lib {
struct Tag {};
int probe(Tag, double) { return 1; }
}
template <class... T> int count() { return probe(lib::Tag{}, sizeof...(T)); }
namespace lib {
int probe(Tag, decltype(sizeof 0)) { return 2; }
}
int main() { return count<int, char>() == 1 ? 0 : 1; }
