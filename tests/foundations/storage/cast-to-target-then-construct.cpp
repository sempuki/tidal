// tidal-rules: STG-16 STG-5
// tidal-expect: run
// tidal-flags: -O0 -fsanitize=undefined -fno-sanitize-recover=undefined
// tidal-note: The cast gives a Point* that still points to the bytes (STG-16). The program converts that pointer to void* for placement new and uses only the pointer that placement new returns.
#include "placement.h"
struct Point {
  int x, y;
  Point(int a, int b) : x(a), y(b) {}
};
int main() {
  alignas(Point) unsigned char buffer[sizeof(Point)];
  Point* address = reinterpret_cast<Point*>(buffer);
  void* place = static_cast<void*>(address);
  Point* made = new (place) Point(2, 5);
  bool same = place == static_cast<void*>(buffer);
  int sum = made->x + made->y;
  made->~Point();
  return same && sum == 7 ? 0 : 1;
}
