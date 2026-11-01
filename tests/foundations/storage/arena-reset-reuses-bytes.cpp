// tidal-rules: STG-6 STG-5 STG-8
// tidal-expect: run
// tidal-flags: -O0 -fsanitize=address,undefined -fno-sanitize-recover=all
// tidal-note: After reset, the bytes that held the Point hold no objects (STG-6). The arena creates the int elements with placement new before it writes them.
#include "placement.h"
struct Point { int x, y; Point(int a, int b) : x(a), y(b) {} ~Point() {} };
class Arena {
  alignas(16) unsigned char bytes[64];
  decltype(sizeof 0) used = 0;
public:
  template <class T, class... Args> T* make(Args... args) {
    used = (used + alignof(T) - 1) & ~(alignof(T) - 1);
    T* made = new (bytes + used) T(args...);
    used += sizeof(T);
    return made;
  }
  template <class T> T* make_array(int n) {
    used = (used + alignof(T) - 1) & ~(alignof(T) - 1);
    T* first = new (bytes + used) T[n]();
    used += sizeof(T) * n;
    return first;
  }
  void reset() { used = 0; }
};
template <class T> void destroy_all(T* first, int n) {
  while (n > 0) first[--n].~T();       // also for a scalar T
}
int main() {
  Arena arena;
  Point* p = arena.make<Point>(3, 4);
  int sum = p->x + p->y;
  p->~Point();
  arena.reset();
  int* counts = arena.make_array<int>(4); // new ints in the bytes of the Point
  for (int i = 0; i != 4; ++i) counts[i] = i;
  sum += counts[3];
  destroy_all(counts, 4);
  return sum == 10 ? 0 : 1;
}
