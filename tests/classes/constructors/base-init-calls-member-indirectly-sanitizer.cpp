// tidal-rules: CTOR-12
// tidal-expect: run-fail
// tidal-flags: -O0 -fsanitize=vptr -fno-sanitize-recover=all
// tidal-expect-clang: run
// tidal-error-gcc: member call on address .* which does not point to an object of type
// tidal-note: The mem_init of Base calls first_of, which calls a member function of Derived before Base is initialized. A call made indirectly from the mem_init is undefined in the same way as a direct call. The Clang sanitizer does not check this call.
// tidal-note: Run-time check with the sanitizer (decision 0028).
struct Base {
  int start;
  explicit Base(int s) : start(s) {}
  virtual ~Base() {}
};
struct Derived;
int first_of(const Derived* d);
struct Derived : Base {
  Derived() : Base(first_of(this)) {}
  int first() const { return 1; }
};
int first_of(const Derived* d) { return d->first(); }
int main() {
  Derived derived;
  return derived.start == 1 ? 0 : 1;
}
