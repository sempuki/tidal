// tidal-rules: INIT-1
// tidal-expect: accept
// tidal-interpreter: run-fail
// tidal-note: x has no initializer, so the read is undefined. No sanitizer in the matrix stops the read, so the matrix compilers only build the program. The interpreter must stop with INIT-1 (decision 0053).
int main() {
  int x;
  int y = x;   // undefined: x has no value
  return y == 0 ? 0 : 1;
}
