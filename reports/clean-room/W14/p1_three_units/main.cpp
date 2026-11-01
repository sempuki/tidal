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
