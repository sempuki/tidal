// tidal-rules: INL-5
// tidal-expect: run
// tidal-sources: static-local-member-shared.tu.cpp
// tidal-note: The function is a member function defined in its class, so it is inline. Both units define the class, and its static local variable is one object.
struct Counter {
  static int tick() { static int n = 0; return ++n; }
};
int other();
int main() { Counter::tick(); other(); return Counter::tick() == 3 ? 0 : 1; }
