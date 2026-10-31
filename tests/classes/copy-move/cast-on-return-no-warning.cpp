// tidal-rules: CPY-13
// tidal-expect: accept
// tidal-flags: -Wall -Wextra -Werror
struct Item { Item() {} Item(const Item&) {} Item(Item&&) {} };
Item&& take(Item& item) { return static_cast<Item&&>(item); }
Item make() {
  Item item;
  return static_cast<Item&&>(item);
}
Item make_other() {
  Item item;
  return take(item);
}
