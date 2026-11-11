// tidal-rules: JMP-6 LOOP-9
// tidal-expect: reject
// tidal-error-clang: cannot jump from this goto statement to its label
// tidal-error-gcc: crosses initialization
// tidal-error-edg: bypasses initialization
// tidal-note: Gap review RV7-A-7. The goto enters the while body and skips the declaration int value = next() in the condition, which is in scope at the label. Every compiler rejects the jump (writer probe roadmap-11/writer-probes/rv7-points.json).
int next();
int run() {
  goto inside;
  while (int value = next()) {
  inside:
    return value;
  }
  return 0;
}
