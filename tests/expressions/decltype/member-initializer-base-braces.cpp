// tidal-rules: DTY-3
// tidal-expect: run
// tidal-expect-gcc: reject
// tidal-error-gcc: expected unqualified-id before .\{.
// tidal-note: Q-0450. GCC rejects a braced list after a decltype base name in a member initializer list. Clang and EDG accept it.
struct Base { int v; Base(int x) : v(x) {} };
Base sample(0);
struct Derived : decltype(sample) {
  Derived() : decltype(sample){8} {}
};
int main() { return Derived().v == 8 ? 0 : 1; }
