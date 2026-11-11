int sum(int n) {
  int total;  // never set
  for (int i = 0; i < n; ++i) total += i;
  return total;
}
int main() { return sum(4) == 6 ? 0 : 1; }
