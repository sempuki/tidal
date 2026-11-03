// tidal-rules: UNW-5 DTOR-4
// tidal-expect: run
// tidal-note: Write report W10. ~Outer has no specifier, and a member's destructor is noexcept(false). So ~Outer can throw, and the handler catches the exception.
struct Loud { ~Loud() noexcept(false) {} };
struct Outer {
  Loud part;
  ~Outer() { throw 1; }
};
static_assert(!noexcept(static_cast<Outer*>(nullptr)->~Outer()));
int main() {
  try { Outer o; }
  catch (int v) { return v == 1 ? 0 : 2; }
  return 3;
}
