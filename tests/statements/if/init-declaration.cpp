// tidal-rules: IF-7 IF-9
// tidal-expect: run
int main() {
  int r = 0;
  if (int a = 3, b = 4; a > b) r = a;
  else r = a + b;
  return r == 7 ? 0 : 1;
}
