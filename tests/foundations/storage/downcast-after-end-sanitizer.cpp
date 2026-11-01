// tidal-rules: STG-22
// tidal-expect: run-fail
// tidal-flags: -O0 -fsanitize=vptr -fno-sanitize-recover=all
// tidal-expect-clang: run
// tidal-error-gcc: downcast of address .* which does not point to an object of type
// tidal-note: The Clang sanitizer does not check this static_cast.
// tidal-note: Run-time check with the sanitizer (decision 0028).
struct Base { virtual ~Base() {} };
struct Derived : Base { int extra = 0; };
int main() {
  Base* base = new Derived;
  base->~Base();
  Derived* derived = static_cast<Derived*>(base);   // the object has ended
  ::operator delete(base);
  return derived != nullptr ? 0 : 1;
}
