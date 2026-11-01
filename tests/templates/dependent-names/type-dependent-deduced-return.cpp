// tidal-rules: DEP-2 DEP-9
// tidal-expect: run
namespace shop { struct Item {}; }
template <class T> struct Box {
  static auto make() { return T{}; }
  int run() { return price(make()); }
};
namespace shop { int price(Item) { return 7; } }
int main() { return Box<shop::Item>{}.run() == 7 ? 0 : 1; }
