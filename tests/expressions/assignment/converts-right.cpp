// tidal-rules: ASN-1
// tidal-expect: run
int main() {
  int i = 0;
  i = 3.9;
  double d = 0;
  d = 2;
  bool b = false;
  b = 5;
  return i == 3 && d == 2.0 && b == true ? 0 : 1;
}
