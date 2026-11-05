// Unit a, written by hand as an implementation of P15 could compile it:
//   int first_in_a = seven;
//   int config = first_in_a * 2;
#include <cstdio>
#include <cstdlib>
volatile int seven = 7;
int first_in_a;
int config;
static int a_done = 0;          // initializers of unit a that have completed
static int a_running = -1;      // initializer now running, or -1
void a_init_upto(int k) {       // the unit's prefix entry
  while (a_done < k) {
    if (a_running == a_done) { std::fprintf(stderr, "cycle in unit a\n"); std::fflush(stderr); std::abort(); }
    a_running = a_done;
    switch (a_done) {
      case 0: first_in_a = seven; break;
      case 1: config = first_in_a * 2; break;
    }
    a_running = -1;
    ++a_done;
  }
}
static int a_start = (a_init_upto(2), 0);   // the unit's own start-up
