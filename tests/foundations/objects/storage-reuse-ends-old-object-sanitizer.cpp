// tidal-rules: OBJ-10 OBJ-2
// tidal-expect: run-fail
// tidal-flags: -O0 -fsanitize=vptr -fno-sanitize-recover=all
// tidal-error: member call on address .* which does not point to an object of type
// tidal-note: The New object reuses the storage of the Old object, so old refers to an object whose lifetime has ended (OBJ-10, a case of OBJ-2). The call reads data through old.
// tidal-note: Run-time check with the sanitizer (decision 0028).
#include "../storage/placement.h"
struct Old {
  virtual void tag() {}
  int data = 1;
  int value() const { return data; }
};
struct New {
  virtual void tag() {}
  int data = 2;
};
int main() {
  alignas(Old) unsigned char buffer[sizeof(Old)];
  Old& old = *new (buffer) Old;
  new (buffer) New;                  // reuses the storage, so old ends
  return old.value() == 1 ? 0 : 1;   // undefined: reads through old
}
