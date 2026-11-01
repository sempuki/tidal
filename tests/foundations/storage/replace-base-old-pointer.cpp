// tidal-rules: STG-19
// tidal-expect: run
// tidal-expect-gcc-16-o2: run-fail
// tidal-note: At -O2, GCC calls Derived::id through the old pointer to the base subobject, so the program exits 1. STG-19 allows any result.
#include "placement.h"
struct Base { virtual int id() { return 1; } };
struct Derived : Base { int id() override { return 2; } };
int main() {
  Derived derived;
  Base* base = &derived;
  base->~Base();
  new (base) Base;   // a complete Base in the storage of a base subobject
  return base->id() == 1 ? 0 : 1;
}
