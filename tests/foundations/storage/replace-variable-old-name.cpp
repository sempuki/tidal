// tidal-rules: STG-9
// tidal-expect: run
#include "placement.h"
int main() {
  int count = 1;
  int* pointer = &count;
  int& reference = count;
  new (&count) int(5);
  return count == 5 && *pointer == 5 && reference == 5 ? 0 : 1;
}
