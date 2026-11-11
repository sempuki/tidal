// tidal-rules: JMP-6 LOOP-9
// tidal-expect: reject
// tidal-error-clang: cannot jump from this goto statement to its label
// tidal-error-gcc: crosses initialization
// tidal-error-edg: bypasses initialization
// tidal-note: Gap review RV7-A-7. A range-based for declares variables for its range and its iterators, each with an initializer, and they are in scope in the body. The goto into the body skips them. Every compiler rejects the jump, also with a reference loop variable and with a case label in place of the goto (writer probe roadmap-11/writer-probes/rv7-points.json).
int items[3] = {1, 2, 3};
int run() {
  goto inside;
  for (int item : items) {
  inside:
    return 1;
  }
  return 0;
}
