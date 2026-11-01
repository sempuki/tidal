// tidal-rules: VIRT-12 NEW-5
// tidal-expect: run-fail
// tidal-flags: -O0 -fsanitize=address -fsized-deallocation
// tidal-error: AddressSanitizer: new-delete-type-mismatch
// tidal-note: Run-time check with the sanitizer (decision 0028). Clang 18 passes the size to delete only under -fsized-deallocation. The destructor is virtual, and delete[] through a pointer to the base is still undefined.
// tidal-note: VIRT-12 is the specific rule, and NEW-5 is the general rule that names it (decision 0052).
struct Base { int a = 0; virtual ~Base() {} };
struct Derived : Base { long more[8] = {}; };
int main() {
  Base* items = new Derived[2];
  delete[] items;   // undefined: the array holds Derived objects
  return 0;
}
