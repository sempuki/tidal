// tidal-rules: STG-22
// tidal-expect: reject
// tidal-error-gcc: accessing .Slot::item. member instead of (initialized|active) .Slot::raw. member
// tidal-error-clang: member call on member .item. of union with active member .raw.
// tidal-error-edg: expression must have a constant value
struct Item {
  int value;
  constexpr int get() const { return value; }
};
union Slot { int raw; Item item; };
constexpr int call() {
  Slot slot{.raw = 1};
  Item* item = &slot.item;   // storage for an Item, with no Item in it
  return item->get();
}
static_assert(call() == 0);
