// tidal-rules: STG-4 STG-5 STG-6
// tidal-expect: run
#include "placement.h"
struct Point {
  int x, y;
  Point(int a, int b) : x(a), y(b) {}
};
int main() {
  alignas(Point) unsigned char buffer[sizeof(Point)];
  Point* point = new (buffer) Point(3, 4);
  int sum = point->x + point->y;
  point->~Point();
  int* number = new (buffer) int(0);
  bool zero_bytes = true;
  for (unsigned i = 0; i != sizeof(int); ++i)
    if (buffer[i] != 0) zero_bytes = false;   // the buffer is still alive
  return sum == 7 && *number == 0 && zero_bytes ? 0 : 1;
}
