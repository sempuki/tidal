// tidal-rules: JMP-14 JMP-5
// tidal-expect: run
// tidal-note: X24-7. A member function of a local class is a separate function, so it can use a label name that its enclosing function also uses.
int count_steps() {
  struct Inner {
    int run() {
      int k = 0;
    again:
      if (++k < 3) goto again;
      return k;
    }
  };
  int steps = 0;
again:
  steps += Inner{}.run();
  if (steps < 6) goto again;
  return steps;
}
int main() { return count_steps() == 6 ? 0 : 1; }
