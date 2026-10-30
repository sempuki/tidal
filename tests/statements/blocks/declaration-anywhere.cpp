// tidal-rules: STMT-2
// tidal-expect: run
int main() {
  int a = 1;
  a += 1;
  int b = a * 2;
  return b == 4 ? 0 : 1;
}
