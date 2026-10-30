// tidal-rules: ASN-1
// tidal-expect: run
int main() {
  int a = 4;
  a = a;
  a += a;
  return a == 8 ? 0 : 1;
}
