// tidal-rules: STG-19 STG-6
// tidal-expect: run-fail
// tidal-flags: -O0 -fsanitize=vptr -fno-sanitize-recover=all
// tidal-error: member call on address .* which does not point to an object of type
// tidal-note: Run-time check with the sanitizer (decision 0028).
#include "placement.h"
struct Base { virtual int id() { return 1; } };
struct Derived : Base { int id() override { return 2; } };
int main() {
  Derived derived;
  Base* base = &derived;
  base->~Base();
  new (base) Base;        // reuses storage of derived, which ends
  return derived.id();
}
