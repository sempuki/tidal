// tidal-rules: STG-19
// tidal-expect: run-fail
// tidal-flags: -O0 -fsanitize=vptr -fno-sanitize-recover=all
// tidal-error: member call on address .* which does not point to an object of type
// tidal-note: Run-time check with the sanitizer (decision 0028).
#include "placement.h"
struct Base { virtual int id() { return 1; } };
struct Derived : Base { int id() override { return 2; } };
struct Other : Base { int id() override { return 3; } };
int main() {
  Derived derived;
  derived.~Derived();
  new (&derived) Other;   // another type, so the name keeps the ended object
  return derived.id();
}
