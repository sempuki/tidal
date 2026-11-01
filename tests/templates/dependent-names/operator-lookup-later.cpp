// tidal-rules: DEP-9
// tidal-expect: run
template <class T> int add(T a, T b) { return a + b; }
namespace shop { struct Item {}; int operator+(Item, Item) { return 7; } }
int main() { return add(shop::Item{}, shop::Item{}) == 7 ? 0 : 1; }
