// tidal-rules: ASN-3
// tidal-expect: run
int main() {
  int a = 1, b = 2, c = 3;
  a = b = c;
  return a == 3 && b == 3 ? 0 : 1;
}
