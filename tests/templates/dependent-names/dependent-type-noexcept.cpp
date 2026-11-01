// tidal-rules: DEP-1 DEP-9
// tidal-expect: run
namespace shop { struct Item {}; }
template <bool B> int run() {
  void (*p)(shop::Item) noexcept(B) = nullptr;
  return price(p);
}
namespace shop { int price(void (*)(Item)) { return 7; } }
int main() { return run<true>() == 7 ? 0 : 1; }
