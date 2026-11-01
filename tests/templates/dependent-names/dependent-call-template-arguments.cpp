// tidal-rules: DEP-8 DEP-9
// tidal-expect: run
namespace lib {
struct Tag {};
}
template <class T> int use() { return probe<T>(lib::Tag{}); }
namespace lib {
template <class U> int probe(Tag) { return sizeof(U); }
}
int main() { return use<int>() == sizeof(int) ? 0 : 1; }
