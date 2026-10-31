// tidal-rules: UN-7
// tidal-expect: run-fail
// tidal-flags: -O0 -fsanitize=undefined -fno-sanitize-recover=undefined
// tidal-error: load of null pointer
// tidal-note: Run-time check with the sanitizer (decision 0028).
int main(int argc, char**) {
  int value = 0;
  int* volatile p = argc > 0 ? nullptr : &value;
  return *p;
}
