// tidal-rules: TR-4 INC-1 INC-4 ODR-7 ODR-8 ODR-10 LNK-1 NS-6 INL-1 LNK-4
// tidal-expect: run
// tidal-sources: area.tu.cpp perimeter.tu.cpp
// tidal-note: Write suite W14, program p1_three_units, units main.cpp, area.cpp, perimeter.cpp (reports/clean-room/W14/report.md). The rules are the ones the report's notes made before writing give for this program.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
#include "shapes.h"

int main() {
  shapes::Rect r{3, 4};
  if (shapes::area(r) != 12) return 1;
  if (shapes::perimeter(r) != 14) return 2;
  if (shapes::is_square(r)) return 3;
  shapes::Rect s{5, 5};
  if (!shapes::is_square(s)) return 4;
  return 0;
}
