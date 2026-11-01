// tidal-rules: STG-4 STG-5 LSUP-9
// tidal-expect: run
#include "byte.h"
#include "placement.h"
struct Reading { double value; long count; };
int main() {
  alignas(Reading) std::byte storage[sizeof(Reading) * 2];
  Reading* low = new (storage) Reading{1.5, 1};
  Reading* high = new (storage + sizeof(Reading)) Reading{2.5, 2};
  return low->value + high->value == 4.0 && low->count + high->count == 3 ? 0 : 1;
}
