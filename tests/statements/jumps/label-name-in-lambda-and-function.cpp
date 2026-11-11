// tidal-rules: JMP-14 JMP-5
// tidal-expect: run
// tidal-note: X24-7. A lambda body is a separate function, so it can use a label name that its enclosing function also uses.
int count_steps() {
  int steps = 0;
  int round = 0;
again:
  auto inner = [&] {
    int k = 0;
  again:
    ++steps;
    if (++k < 3) goto again;
  };
  inner();
  if (++round < 2) goto again;
  return steps;
}
int main() { return count_steps() == 6 ? 0 : 1; }
