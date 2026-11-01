// tidal-rules: DEP-9
// tidal-expect: run
namespace lib {
struct Item {};
int probe(Item, double) { return 1; }
}
template <class T> int use(T item) { return probe(item, 0); }
namespace lib {
int probe(Item, int) { return 2; }
}
int main() { return use(lib::Item{}) == 2 ? 0 : 1; }
