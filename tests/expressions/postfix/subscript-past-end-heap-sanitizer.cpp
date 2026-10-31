// tidal-rules: PFX-4
// tidal-expect: run-fail
// tidal-flags: -fsanitize=address
// tidal-error: AddressSanitizer: heap-buffer-overflow
// tidal-note: Run-time check with the sanitizer (decision 0028).
int main(int argc, char**) {
  int* cells = new int[4]{};
  volatile int i = 3 + argc;
  int v = cells[i];
  delete[] cells;
  return v;
}
