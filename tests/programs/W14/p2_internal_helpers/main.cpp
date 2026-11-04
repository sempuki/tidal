// tidal-rules: LNK-2 LNK-5 NS-4 NS-13 TR-4 NAM-8 ODR-9 ODR-11
// tidal-expect: run
// tidal-sources: stats.tu.cpp
// tidal-note: Write suite W14, program p2_internal_helpers, units main.cpp, stats.cpp (reports/clean-room/W14/report.md). The rules are the ones the report's notes made before writing give for this program.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
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
