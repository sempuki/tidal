// tidal-rules: COMMA-1
// tidal-expect: run
int main() {
  int i = 1;
  int r = (i++, i * 10);
  return r == 20 ? 0 : 1;
}
