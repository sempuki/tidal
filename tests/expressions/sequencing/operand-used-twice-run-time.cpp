// tidal-rules: SEQ-11
// tidal-expect: run
// tidal-note: The constructs run at run time. A compound shift and a postfix increment each evaluate their left operand once, and a range-based for evaluates its range expression once.
int calls = 0;
int cells[3] = {1, 2, 3};
int& at() { ++calls; return cells[0]; }
int (&all())[3] { ++calls; return cells; }
int main() {
  at() <<= 2;
  at()++;
  if (calls != 2 || cells[0] != 5) return 1;
  calls = 0;
  int sum = 0;
  for (int v : all()) sum += v;
  return calls == 1 && sum == 10 ? 0 : 2;
}
