// tidal-rules: CTOR-12
// tidal-expect: run-fail
// tidal-flags: -O0 -fsanitize=vptr -fno-sanitize-recover=all
// tidal-expect-clang: run
// tidal-error-gcc: member call on address .* which does not point to an object of type
// tidal-note: The mem_init of Base calls a member function of Derived before Base is initialized. The Clang sanitizer does not check this call.
// tidal-note: Run-time check with the sanitizer (decision 0028).
struct Base {
  int start;
  explicit Base(int s) : start(s) {}
  virtual ~Base() {}
};
struct Derived : Base {
  Derived() : Base(first()) {}   // undefined: Base is not yet initialized
  int first() const { return 1; }
};
int main() {
  Derived derived;
  return derived.start == 1 ? 0 : 1;
}
