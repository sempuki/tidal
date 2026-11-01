#include "stats.h"

// This unit has its own private `add` and `helper_calls`. They are
// different entities from the ones in stats.cpp (TR-4, LNK-5).
namespace stats {
namespace {
int helper_calls = 100;
int add(int a, int b) { return a * b; }   // deliberately different
}  // namespace
}  // namespace stats

int main() {
  int data[4] = {2, 4, 6, 8};
  if (stats::total(data, 4) != 20) return 1;   // used stats.cpp's add
  if (stats::mean(data, 4) != 5) return 2;
  if (stats::calls() != 8) return 3;          // stats.cpp's counter only
  if (stats::add(3, 4) != 12) return 4;       // this unit's add
  if (stats::helper_calls != 100) return 5;   // this unit's counter
  return 0;
}
