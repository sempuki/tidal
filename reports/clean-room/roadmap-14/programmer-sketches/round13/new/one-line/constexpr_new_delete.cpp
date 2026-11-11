// new and delete inside constant evaluation.
constexpr int sum_squares(int n) {
  int* a = new int[n];
  for (int i = 0; i < n; ++i) a[i] = i * i;
  int s = 0;
  for (int i = 0; i < n; ++i) s += a[i];
  delete[] a;
  return s;
}
static_assert(sum_squares(5) == 30);
int main() { return sum_squares(4) - 14; }
