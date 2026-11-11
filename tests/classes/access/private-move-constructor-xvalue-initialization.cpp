// tidal-rules: ACC-3 CPY-9
// tidal-expect: reject
// tidal-error-clang: calling a private constructor
// tidal-error-gcc: is private within this context
// tidal-error-edg: is inaccessible
// tidal-note: G154 (front-end report 23). The initializer is a Lease xvalue, a non-const rvalue, so overload resolution selects the move constructor (CPY-9). The move constructor is private, so the initialization is not valid (ACC-3). The public copy constructor does not make it valid.
struct Lease {
  Lease() = default;
  Lease(const Lease&) = default;
private:
  Lease(Lease&&) = default;
};
int main() {
  Lease a;
  Lease b = static_cast<Lease&&>(a);
  (void)b;
  return 0;
}
