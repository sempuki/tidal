// tidal-rules: COMMA-4
// tidal-expect: run
int main() {
  int a = 1, b = (2, 3);
  int arr[] = {1, 2};
  return a == 1 && b == 3 && sizeof(arr) == 2 * sizeof(int) ? 0 : 1;
}
