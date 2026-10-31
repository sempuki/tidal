// tidal-rules: JMP-7
// tidal-expect: reject
// tidal-error-clang: break. statement not in loop or switch
// tidal-error-gcc: break statement not within loop or switch
// tidal-error-edg: break statement may only be used within a loop or switch
void f() {
  break;
}
