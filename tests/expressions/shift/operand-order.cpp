// tidal-rules: SHF-8
// tidal-expect: run
int order[2];
int n = 0;
int left() { order[n++] = 1; return 1; }
int right() { order[n++] = 2; return 3; }
int main() {
  int r = left() << right();
  return r == 8 && order[0] == 1 && order[1] == 2 ? 0 : 1;
}
