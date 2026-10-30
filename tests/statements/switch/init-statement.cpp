// tidal-rules: SW-4
// tidal-expect: run
int main() {
  int r = 0;
  switch (int x = 2; x * 2) { case 4: r = x; break; }
  return r == 2 ? 0 : 1;
}
