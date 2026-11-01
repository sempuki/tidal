// tidal-rules: STG-5
// tidal-expect: run
#include "placement.h"
struct Gauge { int level; Gauge() : level(5) {} };
int main() {
  alignas(Gauge) unsigned char buffer[sizeof(Gauge)];
  void* place = buffer;
  Gauge* gauge = new (place) Gauge;
  bool same = static_cast<void*>(gauge) == place;
  int level = gauge->level;
  gauge->~Gauge();
  return same && level == 5 ? 0 : 1;
}
