// tidal-rules: DEFARG-10 DEFARG-3
// tidal-expect: run
// tidal-note: A friend function defined in the class has its default argument checked as if at the end of the class, so the default can name a static member declared after the friend.
struct Gauge {
  friend int level(Gauge, int n = full) { return n; }
  static const int full = 9;
};
int main() { return level(Gauge{}) == 9 ? 0 : 1; }
