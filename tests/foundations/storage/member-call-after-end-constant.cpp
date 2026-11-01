// tidal-rules: STG-22 DTOR-7
// tidal-expect: reject
// tidal-error-gcc: outside its lifetime
// tidal-error-clang: member call on object outside its lifetime
// tidal-error-edg: expression must have a constant value
struct Item {
  int value;
  constexpr int get() const { return value; }
  constexpr ~Item() {}
};
constexpr int call() {
  Item item{1};
  Item* pointer = &item;
  item.~Item();
  return pointer->get();
}
static_assert(call() == 1);
