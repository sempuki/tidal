// tidal-rules: PFX-21
// tidal-expect: run-fail
// tidal-flags: -O0 -fsanitize=undefined -fno-sanitize-recover=undefined
// tidal-error: member call on null pointer
// tidal-note: Run-time check with the sanitizer (decision 0028). The member function reads no member, and the call is still undefined.
struct Gauge { int zero() { return 0; } };
int main(int argc, char**) {
  Gauge g;
  Gauge* volatile p = argc > 0 ? nullptr : &g;
  return p->zero();
}
