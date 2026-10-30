// tidal-rules: ASN-9
// tidal-expect: run
int calls = 0;
int value = 1;
int& target() { ++calls; return value; }
int main() {
  target() += 4;
  target() *= 2;
  return calls == 2 && value == 10 ? 0 : 1;
}
