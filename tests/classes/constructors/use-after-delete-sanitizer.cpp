// tidal-rules: DTOR-7
// tidal-expect: run-fail
// tidal-flags: -O0 -fsanitize=address
// tidal-error: AddressSanitizer: heap-use-after-free
// tidal-note: Run-time check with the sanitizer (decision 0028).
struct Counter { int n; ~Counter() {} };
int main(int argc, char**) {
  Counter* c = new Counter{argc};
  delete c;
  return c->n;
}
