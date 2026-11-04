// tidal-rules: STG-29
// tidal-expect: run
// tidal-flags: -O0 -fsanitize=undefined -fno-sanitize-recover=undefined
// tidal-note: The byte array is a member of Slot. The cast gives a Point* that still points to the first byte (STG-29). Placement new takes the address and creates a Point there. The program reaches the Point through the pointer that placement new returns.
#include "placement.h"
struct Point {
  int x, y;
  Point(int a, int b) : x(a), y(b) {}
};
struct Slot {
  alignas(Point) unsigned char bytes[sizeof(Point)];
  int count = 3;
};
int main() {
  Slot slot;
  Point* address = reinterpret_cast<Point*>(slot.bytes);
  Point* made = new (address) Point(4, 6);
  bool same = static_cast<void*>(made) == static_cast<void*>(slot.bytes);
  int sum = made->x + made->y;
  made->~Point();
  return same && sum == 10 && slot.count == 3 ? 0 : 1;
}
