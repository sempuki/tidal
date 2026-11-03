// DTOR-4 vs UNW-5: ~Outer has no specifier, its member's destructor is
// noexcept(false). Does a throw that leaves ~Outer terminate?
struct Loud { ~Loud() noexcept(false) {} };
struct Outer { Loud l; ~Outer() { throw 1; } };
static_assert(!noexcept(static_cast<Outer*>(nullptr)->~Outer()));
int main() {
  try { Outer o; } catch (int v) { return v == 1 ? 0 : 2; }
  return 3;
}
