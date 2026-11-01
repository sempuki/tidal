// tidal-rules: DEP-1 DEP-5
// tidal-expect: run
namespace shop { struct Item {}; int pick(Item, long = 0) { return 1; } }
template <class T> struct Box {
  using item = shop::Item;
  int get() { item i; return pick(i); }
};
namespace shop { int pick(Item) { return 2; } }
int main() { return Box<int>{}.get() == 1 ? 0 : 1; }
