// tidal-rules: UNW-1
// tidal-expect: run
// tidal-expect-clang: run-fail
// tidal-note: Q-0365. The return statement has built the result, and then the destructor of a local object throws. The result is destroyed before the handler runs. GCC destroys it. Clang never destroys the result, and the program returns 1.
int made = 0;
int ended = 0;
struct R { R() { ++made; } R(const R&) { ++made; } ~R() { ++ended; } };
struct T { ~T() noexcept(false) { throw 1; } };
R named() { T t; R r; return r; }
R fresh() { T t; return R(); }
int main() {
  try { R x = named(); } catch (int) {}
  try { R y = fresh(); } catch (int) {}
  return made == ended ? 0 : 1;
}
