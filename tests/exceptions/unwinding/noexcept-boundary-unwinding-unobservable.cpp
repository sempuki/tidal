// tidal-rules: UNW-7
// tidal-expect: run-fail
volatile int destroyed = 0;
struct D { ~D() { destroyed = destroyed + 1; } };
void thrower() { D inner; throw 0; }
void closed() noexcept { D outer; thrower(); }  // the exception reaches the noexcept boundary
int main() {
  try { closed(); } catch (...) {}  // no handler outside closed takes part
  return 0;
}
