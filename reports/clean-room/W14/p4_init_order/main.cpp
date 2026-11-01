#include "counter.h"

int main() {
  // Calling a function of each unit is a use of that unit, so its
  // start-up initialization has happened by now even if the
  // implementation defers it (SINIT-5).
  int a = tickets::ticket_of_a();
  int b = tickets::ticket_of_b();
  // The two units ran in some order: one got 100 and the other 101.
  if (a + b != 201) return 1;
  if (a != 100 && a != 101) return 2;
  if (tickets::counter().issued != 2) return 3;
  if (tickets::counter().take() != 102) return 4;
  return 0;
}
