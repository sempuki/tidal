// tidal-rules: STG-19
// tidal-expect: accept
// tidal-interpreter: run-fail
// tidal-note: STG-19 makes the call through the old pointer undefined, so the matrix compilers only build the program. No sanitizer in the matrix stops the run. The interpreter must stop with STG-19 (decision 0053).
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
