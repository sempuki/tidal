// tidal-rules: ASN-5
// tidal-expect: run
// tidal-note: Gap review 2026-10-30.
int main() {
  bool b = false;
  b += 2;
  return b == true ? 0 : 1;
}
