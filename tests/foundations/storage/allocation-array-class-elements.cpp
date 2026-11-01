// tidal-rules: STG-28 STG-5 STG-7 STG-8 STG-10
// tidal-expect: run
// tidal-flags: -O0 -fsanitize=address,undefined -fno-sanitize-recover=all
// tidal-note: Item has a user-provided destructor, so it is not an implicit-lifetime type. The call creates an Item[4] whose elements have not begun their lifetimes, and the cast result points to element 0 (Q-0323).
#include "placement.h"
int alive = 0;
struct Item {
  int value;
  explicit Item(int v) : value(v) { ++alive; }
  ~Item() { --alive; }
};
int main() {
  void* raw = ::operator new(sizeof(Item) * 4);
  Item* items = static_cast<Item*>(raw);
  Item* last = items + 3;              // no element has begun its lifetime yet
  new (last) Item(40);
  for (int i = 0; i != 3; ++i) new (items + i) Item(i * 10);
  int sum = 0;
  for (Item* p = items; p != items + 4; ++p) sum += p->value;
  for (int i = 4; i != 0; --i) items[i - 1].~Item();
  ::operator delete(raw);
  return sum == 70 && alive == 0 ? 0 : 1;
}
