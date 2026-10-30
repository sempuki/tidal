// tidal-rules: IF-12
// tidal-expect: run
int main() {
  int r = 0;
  goto inside;
  if (false) {
  inside:
    r = 1;
  }
  return r == 1 ? 0 : 1;
}
