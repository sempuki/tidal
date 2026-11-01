int count(const int a[9]) { return sizeof(a) / sizeof(a[0]); }
int main() {
  int d[9] = {};
  return count(d) == 9 ? 0 : 1;
}
