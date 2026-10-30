// tidal-rules: ASN-5
// tidal-expect: run
int main() {
  int arr[4] = {1, 2, 3, 4};
  int* p = arr;
  p += 3;
  p -= 1;
  return *p == 3 ? 0 : 1;
}
