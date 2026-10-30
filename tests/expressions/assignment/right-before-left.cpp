// tidal-rules: ASN-4
// tidal-expect: run
int order[2];
int n = 0;
int slot = 0;
int& left() { order[n++] = 1; return slot; }
int right() { order[n++] = 2; return 7; }
int main() {
  left() = right();
  return slot == 7 && order[0] == 2 && order[1] == 1 ? 0 : 1;
}
