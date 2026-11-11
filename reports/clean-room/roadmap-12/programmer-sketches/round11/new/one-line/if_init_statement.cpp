// if and switch with an init statement.
int find(const int* a, int n, int v) { for (int i = 0; i < n; ++i) if (a[i] == v) return i; return -1; }
int main() {
  int a[] = {4, 7, 9};
  if (int i = find(a, 3, 7); i >= 0) return i == 1 ? 0 : 1;
  else return 2;
}
