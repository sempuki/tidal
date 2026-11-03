// tidal-rules: UNW-10 UNW-5
// tidal-expect: run-fail
// tidal-error: exit (-6|134) /
// tidal-note: Write report W10. Clang and GCC stop the program with SIGABRT, which the runner reports as exit -6 on the host and 134 in a container. If the static object were destroyed, its destructor would end the program with status 0 and the test would fail.
extern "C" void _Exit(int);
struct Global { ~Global() { _Exit(0); } } global;
void fail() noexcept { throw 1; }
int main() {
  fail();
  return 0;
}
