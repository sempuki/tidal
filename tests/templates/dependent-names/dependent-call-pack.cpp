// tidal-rules: DEP-8 DEP-9
// tidal-expect: run
namespace lib {
struct Item {};
}
template <class... T> int use(T... values) { return probe(values...); }
namespace lib {
int probe(Item) { return 7; }
}
int main() { return use(lib::Item{}) == 7 ? 0 : 1; }
