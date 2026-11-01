// tidal-rules: DEP-3 DEP-5
// tidal-expect: run
namespace lib {
struct Tag {};
int probe(Tag, double) { return 1; }
}
template <class T> int by_size() { return probe(lib::Tag{}, sizeof(T)); }
template <class T> int by_cast(T v) { return probe(lib::Tag{}, static_cast<int>(v)); }
namespace lib {
int probe(Tag, decltype(sizeof 0)) { return 2; }
int probe(Tag, int) { return 3; }
}
int main() { return by_size<int>() == 1 && by_cast(0L) == 1 ? 0 : 1; }
