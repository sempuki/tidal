// tidal-rules: IF-1
// tidal-expect: run
int main() {
  int r = 0;
  if (true) r += 1; else r += 10;
  if (false) r += 100; else r += 1000;
  if (false) r += 10000;
  return r == 1001 ? 0 : 1;
}
