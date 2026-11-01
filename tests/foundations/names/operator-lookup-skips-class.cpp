// tidal-rules: NAM-38 OVL-7
// tidal-expect: run
// tidal-note: Inside Outer, lookup for a + b skips the class scope of Outer, so the member operator+ does not hide the global one.
struct Value { int n; };
int operator+(Value a, Value b) { return a.n + b.n; }
struct Outer {
  int operator+(int) const { return 100; }
  static int add(Value a, Value b) { return a + b; }
};
int main() { return Outer::add(Value{1}, Value{2}) == 3 ? 0 : 1; }
