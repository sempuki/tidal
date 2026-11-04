// tidal-rules: CTOR-12
// tidal-expect: run-fail
// tidal-flags: -O0 -fsanitize=vptr -fno-sanitize-recover=all
// tidal-expect-clang: run
// tidal-error-gcc: member call on address .* which does not point to an object of type
// tidal-note: The mem_init of Base calls a member function of the object under construction through a pointer to it, before Base is initialized. The call is undefined as a call through this would be. The Clang sanitizer does not check this call.
// tidal-note: Run-time check with the sanitizer (decision 0028).
struct Base {
  int start;
  explicit Base(int s) : start(s) {}
  virtual ~Base() {}
};
struct Derived;
int first_of(Derived* d);
struct Derived : Base {
  Derived() : Base(first_of(this)) {}   // undefined: Base is not yet initialized
  int first() const { return 1; }
};
int first_of(Derived* d) { return d->first(); }
int main() {
  Derived derived;
  return derived.start == 1 ? 0 : 1;
}
