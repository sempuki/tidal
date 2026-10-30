// tidal-rules: ASN-4
// tidal-expect: run
int main() {
  int i = 1;
  i = i++ + 1;
  return i == 2 ? 0 : 1;
}
