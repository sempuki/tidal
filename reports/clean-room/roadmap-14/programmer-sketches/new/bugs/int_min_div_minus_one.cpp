int ratio(int a, int b) { return a / b; }
int main() {
  int lo = -2147483647 - 1;
  return ratio(lo, -1) > 0 ? 0 : 1;
}
