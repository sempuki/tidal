// tidal-rules: STMT-2
// tidal-expect: run
int main() {
  int x = 1;
  {
    x = 2;
    int x = 3;
    {
      int y = x * 10;
      int x = y + 1;
      if (x != 31) return 1;
    }
    if (x != 3) return 2;
  }
  return x == 2 ? 0 : 3;
}
