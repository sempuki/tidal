// tidal-rules: REF-5 OBJ-2
// tidal-expect: run-fail
// tidal-flags: -O0 -fsanitize=address
// tidal-error: AddressSanitizer: stack-use-after-scope
// tidal-note: Run-time check with the sanitizer (decision 0028). The temporary binds to the parameter of pass, so the full-expression ends its life.
// tidal-note: REF-5 is the specific rule, and OBJ-2 is the general rule that names it (decision 0052).
const int& pass(const int& x) { return x; }
int main(int argc, char**) {
  const int& kept = pass(argc + 1);
  return kept == 2 ? 0 : 1;   // undefined: the temporary is gone
}
