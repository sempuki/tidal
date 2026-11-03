// tidal-rules: STG-9
// tidal-expect: run
#include "placement.h"
int ended = 0;
struct Counter {
  int v;
  Counter(int x) : v(x) {}
  ~Counter() { ++ended; }        // not trivial
};
int main() {
  {
    Counter c(1);
    c.~Counter();                // ended == 1
    ::new (&c) Counter(2);       // c now names the new object
    if (c.v != 2) return 1;
  }                              // the block destroys the new object once
  return ended == 2 ? 0 : 2;
}
