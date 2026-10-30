// tidal-rules: JMP-2
// tidal-expect: run
int main() {
  int hits = 0;
  for (int i = 0; i < 3; ++i) {
    switch (i) { case 1: continue; default: break; }
    ++hits;
  }
  return hits == 2 ? 0 : 1;
}
