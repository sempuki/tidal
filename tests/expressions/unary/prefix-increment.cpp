// tidal-rules: UN-20
// tidal-expect: run
int main() {
  int i = 1;
  ++i;
  --i;
  ++(++i);
  double d = 0.5;
  ++d;
  int arr[3] = {1, 2, 3};
  int* p = arr;
  ++p;
  return i == 3 && d == 1.5 && *p == 2 ? 0 : 1;
}
