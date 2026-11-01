// tidal-rules: STG-9 LSUP-10
// tidal-expect: accept
#include "construct.h"
struct Item { int value; };
constexpr int replaced() {
  Item items[3]{{1}, {2}, {3}};
  std::construct_at(&items[1], 7);   // becomes element 1 of items
  int sum = 0;
  for (Item* p = items; p != items + 3; ++p) sum += p->value;
  return sum;
}
static_assert(replaced() == 11);
