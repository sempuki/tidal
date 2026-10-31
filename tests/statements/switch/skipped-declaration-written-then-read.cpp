// tidal-rules: JMP-16
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-expect-edg: reject
// tidal-error-gcc: from outside current evaluation
// tidal-error-edg: run-time storage
// tidal-note: Q-0121. Clang lets constant evaluation write a variable whose declaration a switch skipped; GCC and EDG treat the variable as outside the evaluation.
constexpr int pick(int v) {
  switch (v) {
    int x;
  case 1:
    x = 5;
    return x;
  }
  return 0;
}
static_assert(pick(1) == 5);
