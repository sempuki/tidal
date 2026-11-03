// tidal-rules: STO-4
// tidal-expect: run
// tidal-note: A static data member declared thread_local, and a block variable declared thread_local without static. The block variable keeps its value from one call to the next, as a static variable does.
struct Counter {
  static thread_local int total;
};
thread_local int Counter::total = 10;
int next() {
  thread_local int calls = 0;
  return ++calls;
}
int main() {
  Counter::total += 1;
  int a = next();
  int b = next();
  return Counter::total == 11 && a == 1 && b == 2 ? 0 : 1;
}
