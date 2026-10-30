// tidal-rules: IF-12
// tidal-expect: run
int pick(int k) {
  int r = 0;
  switch (k) {
    case 0:
      if (false) {
    case 1:
        r = 7;
      }
  }
  return r;
}
int main() { return pick(1) == 7 && pick(0) == 0 ? 0 : 1; }
