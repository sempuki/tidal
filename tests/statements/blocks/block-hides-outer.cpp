// tidal-rules: STMT-2
// tidal-expect: run
int main() {
  int x = 1;
  {
    int x = 2;
    if (x != 2) return 1;
  }
  return x == 1 ? 0 : 2;
}
