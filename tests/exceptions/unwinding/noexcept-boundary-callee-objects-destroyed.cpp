// tidal-rules: UNW-7
// tidal-expect: run
// tidal-expect-gcc-16-o2: run-fail
// tidal-note: The destructor ends the program with status 0, so the run succeeds only if the object is destroyed. Clang and GCC destroy the objects of a function that the noexcept function calls. GCC 16 at -O2 destroys no object, which UNW-7 also allows.
extern "C" [[noreturn]] void _Exit(int) noexcept;
struct Leaves { ~Leaves() { _Exit(0); } };
void thrower() { Leaves inner; throw 0; }
void closed() noexcept { thrower(); }
int main() {
  try { closed(); } catch (...) {}
  return 1;
}
