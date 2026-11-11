// tidal-rules: JMP-7
// tidal-expect: reject
// tidal-error-clang: break. statement not in loop or switch
// tidal-error-gcc: break statement not within loop or switch
// tidal-error-edg: break statement may only be used within a loop or switch
// tidal-note: X24-6. A member function of a class defined in a loop is a separate function, so the loop does not hold its break.
void f() {
  for (int i = 0; i < 3; ++i) {
    struct Step { void run() { break; } };
    Step{}.run();
  }
}
