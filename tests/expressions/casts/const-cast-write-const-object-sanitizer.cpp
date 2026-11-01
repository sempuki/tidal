// tidal-rules: CAST-5 CV-5
// tidal-expect: run-fail
// tidal-flags: -O0 -fsanitize=address
// tidal-error: AddressSanitizer: SEGV
// tidal-note: Run-time check with the sanitizer (decision 0028). The const object with static storage duration is in storage that cannot be written, so the write faults.
const int limit = 5;
int main(int argc, char**) {
  int* writable = const_cast<int*>(&limit);
  *writable = argc;   // undefined: limit is const
  return 0;
}
