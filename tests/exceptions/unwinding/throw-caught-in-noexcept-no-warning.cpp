// tidal-rules: UNW-8
// tidal-expect: accept
// tidal-flags: -Wall -Werror
struct Base {};
struct Derived : Base {};
int f() noexcept {
  int r = 0;
  try { throw 1; } catch (int) { r += 1; }
  try { throw Derived{}; } catch (Base&) { r += 2; }
  try { throw 2.0; } catch (...) { r += 4; }
  return r;
}
