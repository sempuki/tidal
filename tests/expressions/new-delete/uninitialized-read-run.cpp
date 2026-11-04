// tidal-rules: NEW-3
// tidal-expect: accept
// tidal-interpreter: run-fail
// tidal-note: new int leaves the object with no value, so the read is undefined. No sanitizer in the matrix stops the read, so the matrix compilers only build the program. The interpreter must stop with NEW-3 (decision 0053).
int main() {
  int* p = new int;
  int v = *p;   // undefined: the object has no value
  delete p;
  return v == 0 ? 0 : 1;
}
