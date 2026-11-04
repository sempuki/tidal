// tidal-rules: UNW-7
// tidal-expect: run-fail
// tidal-note: The destructor ends the program with status 0, so the run succeeds only if the object is destroyed. With no matching handler, Clang and GCC destroy no object, and the program stops (UNW-5).
extern "C" [[noreturn]] void _Exit(int) noexcept;
struct Leaves { ~Leaves() { _Exit(0); } };
void thrower() { Leaves kept; throw 0; }
int main() {
  thrower();
  return 1;
}
