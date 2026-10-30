// tidal-rules: REF-7
// tidal-expect: run
int main() {
  long l = 1;
  int&& r = l;
  l = 2;
  return r == 1 ? 0 : 1;
}
