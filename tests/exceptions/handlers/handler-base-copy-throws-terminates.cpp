// tidal-rules: TRY-18
// tidal-expect: run-fail
// tidal-note: The program throws a derived object, and the handler takes the base by value. The copy of the base throws.
struct Base {
  Base() {}
  Base(const Base&) { throw 3; }
};
struct Derived : Base {
  Derived() {}
  Derived(const Derived&) : Base() {}
};
int main() {
  try {
    try { throw Derived{}; }
    catch (Base b) { return 1; }
  } catch (int) { return 0; }
  return 0;
}
