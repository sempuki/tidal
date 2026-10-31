// tidal-rules: DER-17 CAST-14
// tidal-expect: run-fail
// tidal-flags: -O0 -fsanitize=vptr -fno-sanitize-recover=all
// tidal-error: does not point to an object of type
// tidal-note: Run-time check with the sanitizer (decision 0028).
struct Base { virtual ~Base() {} };
struct Left : Base { int x = 1; };
struct Right : Base {};
int main(int argc, char**) {
  Right r;
  Base* volatile b = &r;
  Left* l = static_cast<Left*>(b);
  return l->x == argc;
}
