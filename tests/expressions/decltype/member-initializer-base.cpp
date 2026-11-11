// tidal-rules: DTY-3
// tidal-expect: run
struct Base { int v; Base(int x) : v(x) {} };
Base sample(0);
struct Derived : decltype(sample) {
  Derived() : decltype(sample)(7) {}
};
int main() { return Derived().v == 7 ? 0 : 1; }
