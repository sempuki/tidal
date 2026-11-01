// tidal-rules: DEP-1 DEP-9
// tidal-expect: run
namespace shop { struct Item {}; }
template <class... T> int run() {
  void (*p)(shop::Item, T...) = nullptr;
  return price(p);
}
namespace shop { int price(void (*)(Item)) { return 7; } }
int main() { return run<>() == 7 ? 0 : 1; }
