// tidal-rules: VIRT-12
// tidal-expect: run-fail
// tidal-flags: -O0 -fsanitize=address -fsized-deallocation
// tidal-error: AddressSanitizer: new-delete-type-mismatch
// tidal-note: Run-time check with the sanitizer (decision 0028). Clang 18 passes the size to delete only under -fsized-deallocation. The sanitizer compares the size that delete passes with the size of the allocation.
struct Base { int a = 0; ~Base() {} };
struct Derived : Base { long more[8] = {}; };
int main() {
  Base* base = new Derived;
  delete base;   // undefined: ~Base is not virtual
  return 0;
}
