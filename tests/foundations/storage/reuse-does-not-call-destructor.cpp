// tidal-rules: STG-6
// tidal-expect: run
#include "placement.h"
int destroyed = 0;
struct Noisy {
  int id;
  ~Noisy() { ++destroyed; }
};
int main() {
  alignas(Noisy) unsigned char buffer[sizeof(Noisy)];
  new (buffer) Noisy{1};
  Noisy* second = new (buffer) Noisy{2};   // ends the first object without its destructor
  int before = destroyed;
  second->~Noisy();
  return before == 0 && destroyed == 1 ? 0 : 1;
}
