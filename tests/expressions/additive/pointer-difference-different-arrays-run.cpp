// tidal-rules: ADD-11
// tidal-expect: accept
// tidal-interpreter: run-fail
// tidal-note: first and second are two arrays, so the difference of pointers into them is undefined. The address sanitizer checks such a difference only with a run-time option that a test cannot set, so the matrix compilers only build the program. The interpreter must stop with ADD-11 (decision 0053).
int first[4];
int second[4];
int main() {
  long distance = &second[1] - &first[1];   // undefined: two arrays
  return distance != 0 ? 0 : 1;
}
