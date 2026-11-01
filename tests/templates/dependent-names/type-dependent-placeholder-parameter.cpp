// tidal-rules: DEP-2 DEP-9
// tidal-expect: run
namespace shop { struct Item { int n; }; }
template <auto V> int run() { return price(V); }
namespace shop { constexpr int price(Item i) { return i.n; } }
int main() { return run<shop::Item{7}>() == 7 ? 0 : 1; }
