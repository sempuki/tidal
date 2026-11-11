// tidal-rules: JMP-14
// tidal-expect: reject
// tidal-error-clang: undeclared label .finished
// tidal-error-gcc: label .finished. used but not defined
// tidal-error-edg: label .finished. was referenced but not defined
// tidal-note: X24-6. A member function of a local class is a separate function, so its goto cannot name a label of the enclosing function.
void f() {
  struct Step { void run() { goto finished; } };
  Step{}.run();
finished:;
}
