// tidal-rules: CAST-3
// tidal-expect: run
int main() {
  int y = 1;
  const int* cp = &y;
  int* p = const_cast<int*>(cp);
  *p = 2;
  const volatile int* cvp = const_cast<const volatile int*>(p);
  return y == 2 && cvp == p ? 0 : 1;
}
