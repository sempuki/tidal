// tidal-rules: UN-5
// tidal-expect: run
int main() {
  int x = 1;
  int* p = &x;
  *p = 7;
  return x == 7 && &*p == p ? 0 : 1;
}
