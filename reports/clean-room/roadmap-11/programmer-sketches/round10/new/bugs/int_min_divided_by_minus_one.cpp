// Planted bug: the quotient of the smallest int by -1 does not fit in int.
int divide(int a, int b) { return b == 0 ? 0 : a / b; }
int main() {
  int lo = -2147483647 - 1;
  return divide(lo, -1) > 0 ? 0 : 1;
}
