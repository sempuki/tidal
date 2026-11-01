// tidal-rules: DEP-2 DEP-9
// tidal-expect: run
namespace shop { struct Item {}; }
template <class... T> int run(T... t) { return price((t, ...)); }
namespace shop { int price(Item) { return 7; } }
int main() { return run(shop::Item{}) == 7 ? 0 : 1; }
