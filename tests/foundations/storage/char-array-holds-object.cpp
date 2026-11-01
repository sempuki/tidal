// tidal-rules: STG-4
// tidal-expect: run
// tidal-note: The array of char provides no storage, so the new object ends the array (Q-0305).
#include "placement.h"
int main() {
  alignas(long) char area[sizeof(long)];
  long* value = new (area) long(9);
  return *value == 9 ? 0 : 1;
}
