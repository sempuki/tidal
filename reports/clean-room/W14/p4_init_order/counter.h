// counter.h: a ticket counter that units use while the program starts.
#ifndef W14_P4_COUNTER_H
#define W14_P4_COUNTER_H

namespace tickets {

// The counter has a constructor that is not constexpr, so a namespace-
// scope Counter would be dynamically initialized (SINIT-2), and a unit
// that reads it during its own start-up could see it before its
// constructor runs (SINIT-3).
struct Counter {
  int next;
  int issued;
  Counter();
  int take();
};

// Safe access: the object is a block-scope static inside this function.
// It is initialized the first time control reaches its declaration
// (STO-2), so every caller sees a constructed counter, whatever the
// order of the units.
Counter& counter();

// A plain start value. constinit makes the compiler check that it is
// constant-initialized, so it holds its value before any code runs
// (CINIT-1, SINIT-1). The header declaration carries constinit too
// (CX-P4, CINIT-2).
extern constinit int first_ticket;

// Each unit reports the ticket it took during start-up.
int ticket_of_a();
int ticket_of_b();

}  // namespace tickets

#endif
