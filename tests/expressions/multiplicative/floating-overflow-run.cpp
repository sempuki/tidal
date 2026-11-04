// tidal-rules: MUL-12
// tidal-expect: accept
// tidal-interpreter: run-fail
// tidal-note: Q-0001. The exact product is finite and larger than the largest double, so the run is undefined under the provisional answer. No sanitizer in the matrix stops the run, so the matrix compilers only build the program. The interpreter must stop with MUL-12 (decision 0053).
int main() {
  double large = 1.0e300;
  double product = large * 1.0e300;   // undefined: out of the range of double
  return product > large ? 0 : 1;
}
