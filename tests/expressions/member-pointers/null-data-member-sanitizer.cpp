// tidal-rules: MPTR-6
// tidal-expect: run-fail
// tidal-flags: -O0 -fsanitize=address
// tidal-error: AddressSanitizer: stack-buffer-underflow
// tidal-note: Run-time check with the sanitizer (decision 0028). GCC and Clang give a null pointer to a data member the offset -1, so the read lands before the object.
struct Point { int x; int y; };
int main(int argc, char**) {
  Point point{1, 2};
  int Point::* volatile chosen = argc > 0 ? nullptr : &Point::y;
  int Point::* member = chosen;
  return point.*member;   // undefined: a null pointer to member
}
