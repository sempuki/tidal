// tidal-rules: UNW-7
// tidal-expect: run-fail
// tidal-note: The destructor ends the program with status 0, so the run succeeds only if the object is destroyed. Clang and GCC do not destroy the objects of the noexcept function itself, and the program stops (UNW-5).
extern "C" [[noreturn]] void _Exit(int) noexcept;
struct Leaves { ~Leaves() { _Exit(0); } };
void thrower() { throw 0; }
void closed() noexcept { Leaves outer; thrower(); }
int main() {
  try { closed(); } catch (...) {}
  return 1;
}
