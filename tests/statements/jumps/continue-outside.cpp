// tidal-rules: JMP-7
// tidal-expect: reject
void f() {
  switch (1) { case 1: continue; }
}
