// tidal-rules: UNW-8
// tidal-expect: reject
// tidal-error-clang: non-throwing exception specification but can still throw
// tidal-error-gcc: will always call .terminate
// tidal-flags: -Wall -Werror
void f() noexcept {
  try { throw 1; }
  catch (int) { throw; }
}
