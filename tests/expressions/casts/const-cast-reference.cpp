// tidal-rules: CAST-3
// tidal-expect: run
int main() {
  int y = 1;
  const int& r = y;
  const_cast<int&>(r) = 5;
  return y == 5 ? 0 : 1;
}
