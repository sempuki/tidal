// tidal-rules: SW-1
// tidal-expect: run
int main() {
  int r = 5;
  switch (3) { case 1: r = 1; }
  return r == 5 ? 0 : 1;
}
