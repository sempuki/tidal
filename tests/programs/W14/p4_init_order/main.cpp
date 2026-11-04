// tidal-rules: SINIT-1 SINIT-2 SINIT-3 SINIT-4 SINIT-5 STO-2 STO-10 CINIT-1 CINIT-2
// tidal-expect: run
// tidal-sources: a.tu.cpp b.tu.cpp counter.tu.cpp
// tidal-note: Write suite W14, program p4_init_order, units main.cpp, a.cpp, b.cpp, counter.cpp (reports/clean-room/W14/report.md). The rules are the ones the report's notes made before writing give for this program.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
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
