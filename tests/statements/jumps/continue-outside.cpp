// tidal-rules: JMP-7
// tidal-expect: reject
// tidal-error-clang: continue. statement not in loop statement
// tidal-error-gcc: continue statement not within a loop
// tidal-error-edg: continue statement may only be used within a loop
void f() {
  switch (1) { case 1: continue; }
}
