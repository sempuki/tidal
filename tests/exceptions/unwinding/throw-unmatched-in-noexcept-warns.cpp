// tidal-rules: UNW-8
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: non-throwing exception specification but can still throw
// tidal-flags: -Wall -Werror
// tidal-note: Q-0179. GCC does not warn when no handler of the try block matches the static type.
void f() noexcept {
  try { throw 1; }
  catch (long) {}
}
