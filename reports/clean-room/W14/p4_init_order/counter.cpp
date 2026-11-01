#include "counter.h"

namespace tickets {

constinit int first_ticket = 100;

Counter::Counter() : next(first_ticket), issued(0) {}

int Counter::take() {
  ++issued;
  return next++;
}

Counter& counter() {
  static Counter the_counter;   // constructed on the first call (STO-2)
  return the_counter;
}

}  // namespace tickets
