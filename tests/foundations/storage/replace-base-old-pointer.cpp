// tidal-rules: STG-19
// tidal-expect: accept
// tidal-note: STG-19 makes the call through the old pointer undefined, so the test checks only that the program compiles. No sanitizer in the matrix stops the run.
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
