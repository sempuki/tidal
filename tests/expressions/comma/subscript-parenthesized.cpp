// tidal-rules: COMMA-6
// tidal-expect: run
int main() {
  int arr[3] = {5, 6, 7};
  return arr[(0, 2)] == 7 ? 0 : 1;
}
