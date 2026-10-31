// tidal-rules: CPY-13
// tidal-expect: reject
// tidal-error-clang: prevents copy elision
// tidal-error-gcc: prevents copy elision
// tidal-flags: -Wall -Werror
namespace std {
struct Item { Item() {} Item(const Item&) {} Item(Item&&) {} };
Item&& move(Item& item) { return static_cast<Item&&>(item); }
}
std::Item make() {
  std::Item item;
  return std::move(item);
}
