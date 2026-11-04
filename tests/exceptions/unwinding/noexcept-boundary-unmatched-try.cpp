// tidal-rules: UNW-7
// tidal-expect: run
// tidal-expect-gcc: run-fail
// tidal-note: UNW-7 lets each implementation choose. The destructor ends the program with status 0, so the run succeeds only if the object is destroyed. The noexcept function holds the call in a try block whose handler does not match. Clang destroys the object of the called function. GCC destroys no object, and the program stops (UNW-5).
extern "C" [[noreturn]] void _Exit(int) noexcept;
struct Leaves { ~Leaves() { _Exit(0); } };
void thrower() { Leaves inner; throw 0; }
void closed() noexcept {
  try { thrower(); } catch (long) {}
}
int main() {
  try { closed(); } catch (...) {}
  return 1;
}
