// tidal-rules: ADD-17
// tidal-expect: accept
// tidal-interpreter: run-fail
// tidal-note: Q-0001. The exact sum is finite and larger than the largest double, so the run is undefined under the provisional answer. No sanitizer in the matrix stops the run, so the matrix compilers only build the program. The interpreter must stop with ADD-17 (decision 0053).
int main() {
  double large = 1.0e308;
  double sum = large + large;   // undefined: out of the range of double
  return sum > large ? 0 : 1;
}
