// tidal-rules: PFX-8
// tidal-expect: run-fail
// tidal-flags: -O0 -fsanitize=address
// tidal-error: AddressSanitizer: SEGV on unknown address 0x0+ 
// tidal-note: Run-time check with the sanitizer (decision 0028). The call jumps to address zero.
int one() { return 1; }
int main(int argc, char**) {
  int (*volatile target)() = argc > 0 ? nullptr : one;
  return target();   // undefined: a null function pointer
}
