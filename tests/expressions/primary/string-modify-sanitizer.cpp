// tidal-rules: PRIM-8
// tidal-expect: run-fail
// tidal-flags: -O0 -fsanitize=address
// tidal-error: AddressSanitizer: SEGV
// tidal-note: Run-time check with the sanitizer (decision 0028). The characters of the literal are in storage that cannot be written, so the write faults.
int main(int argc, char**) {
  char* text = const_cast<char*>("text");
  text[0] = static_cast<char>(argc);   // undefined: a character of a literal
  return 0;
}
