// tidal-rules: CV-5
// tidal-expect: run-fail
// tidal-flags: -O0 -fsanitize=address
// tidal-error: AddressSanitizer: SEGV
// tidal-note: Run-time check with the sanitizer (decision 0028). A block-scope static const object with a constant initializer is in storage that cannot be written.
int* writable() {
  static const int kept = 4;
  return const_cast<int*>(&kept);
}
int main(int argc, char**) {
  *writable() = argc;   // undefined: kept is defined const
  return 0;
}
