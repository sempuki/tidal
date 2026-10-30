// tidal-rules: JMP-5
// tidal-expect: run
int main() {
  int n = 0;
  {
    goto inner;
  }
  n = 5;
  {
inner:
    n += 1;
  }
  return n == 1 ? 0 : 1;
}
