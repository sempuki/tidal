// tidal-rules: NEW-5
// tidal-expect: run-fail
// tidal-flags: -O0 -fsanitize=address
// tidal-error: AddressSanitizer: alloc-dealloc-mismatch
// tidal-note: Run-time check with the sanitizer (decision 0028).
int main(int argc, char**) {
  int* p = new int[4]{argc};
  delete p;
}
