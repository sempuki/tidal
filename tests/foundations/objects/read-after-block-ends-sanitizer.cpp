// tidal-rules: OBJ-2 UN-7
// tidal-expect: run-fail
// tidal-flags: -fsanitize=address
// tidal-error: AddressSanitizer: stack-use-after-scope
// tidal-note: Run-time check with the sanitizer (decision 0028).
int main(int argc, char**) {
  int* p;
  {
    int inner = argc;
    p = &inner;
  }
  return *p;
}
