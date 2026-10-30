// tidal-rules: CV-5
// tidal-expect: run
constexpr int f() {
  int y = 1;
  const int& r = y;
  const_cast<int&>(r) = 2;
  return y;
}
static_assert(f() == 2);
int main() { return f() == 2 ? 0 : 1; }
