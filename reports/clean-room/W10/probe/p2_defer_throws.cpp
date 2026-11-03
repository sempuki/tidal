// A cleanup that throws from a guard destructor without a specifier.
template <class F> struct Defer { F f; ~Defer() { f(); } };
int main() {
  try { Defer<void(*)()> d{[] { throw 1; }}; } catch (int) { return 0; }
  return 3;
}
