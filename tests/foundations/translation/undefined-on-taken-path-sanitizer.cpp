// tidal-rules: TR-8 ADD-9
// tidal-expect: run-fail
// tidal-flags: -O0 -fsanitize=undefined -fno-sanitize-recover=undefined
// tidal-error: signed integer overflow
// tidal-note: Run-time check with the sanitizer (decision 0028). The run takes the path with the overflow, and the sanitizer stops it there.
int main(int argc, char**) {
  volatile int largest = 2147483647;
  int sum = 0;
  if (argc > 0) sum = largest + argc;   // undefined: the taken path overflows
  return sum == 0 ? 0 : 1;
}
