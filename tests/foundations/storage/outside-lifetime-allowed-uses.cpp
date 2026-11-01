// tidal-rules: STG-10
// tidal-expect: run
#include "placement.h"
struct Item {
  int value;
  ~Item() {}
};
int main() {
  Item item{1};
  Item* pointer = &item;
  pointer->~Item();
  void* storage = pointer;                                     // allowed
  unsigned char* bytes = static_cast<unsigned char*>(storage); // allowed
  Item& reference = item;                                      // allowed
  bool same = static_cast<void*>(bytes) == static_cast<void*>(&reference);
  new (pointer) Item{2};
  return same && item.value == 2 ? 0 : 1;
}
