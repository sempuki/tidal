// tidal-rules: CAST-9
// tidal-expect: run
int main() {
  int y = 1;
  const int* cp = &y;
  unsigned* p = (unsigned*)cp;
  *p = 4;
  return y == 4 ? 0 : 1;
}
