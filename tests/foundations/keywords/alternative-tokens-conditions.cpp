// tidal-rules: KW-2
// tidal-expect: run
// tidal-note: The alternative spellings in conditions at run time. and and or keep their order of evaluation and stop early.
int calls = 0;
bool hit() { ++calls; return true; }
int main() {
  bool a = false and hit();
  bool b = true or hit();
  if (calls != 0 or a or not b) return 1;
  int v = 5;
  if (v not_eq 5) return 2;
  return (true xor false) and compl 0 == -1 ? 0 : 3;
}
