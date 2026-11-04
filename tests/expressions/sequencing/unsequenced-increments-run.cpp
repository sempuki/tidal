// tidal-rules: SEQ-4
// tidal-expect: accept
// tidal-interpreter: run-fail
// tidal-note: The two increments of count are unsequenced, so the run is undefined (SEQ-4). No sanitizer in the matrix stops the run, so the matrix compilers only build the program. The interpreter must stop with SEQ-4 (decision 0053).
int main() {
  int count = 0;
  int sum = count++ + count++;   // undefined: two unsequenced writes to count
  return sum == 1 ? 0 : 1;
}
