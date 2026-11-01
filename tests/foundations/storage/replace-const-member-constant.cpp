// tidal-rules: STG-9 LSUP-10
// tidal-expect: accept
#include "construct.h"
struct Entry { const int key; int value; };
constexpr int replaced() {
  Entry entry{1, 2};
  Entry* old = &entry;
  std::construct_at(old, 7, 8);
  return entry.key + old->key + old->value;
}
static_assert(replaced() == 22);
