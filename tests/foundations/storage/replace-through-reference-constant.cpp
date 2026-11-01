// tidal-rules: STG-9 LSUP-10
// tidal-expect: accept
#include "construct.h"
struct Item {
  int value;
  constexpr ~Item() {}
};
constexpr int replaced() {
  Item item{1};
  Item& old = item;
  old.~Item();
  std::construct_at(&item, 9);
  return old.value;
}
static_assert(replaced() == 9);
