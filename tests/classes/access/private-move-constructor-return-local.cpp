// tidal-rules: ACC-3 CPY-11
// tidal-expect: reject
// tidal-error-clang: calling a private constructor
// tidal-error-gcc: is private within this context
// tidal-error-edg: is inaccessible
// tidal-note: G154 (front-end report 23). The name of the local l in the return statement is an xvalue (CPY-11), so the return selects the move constructor. The move constructor is private, so the return is not valid (ACC-3). The public copy constructor does not make it valid.
struct Lease {
  Lease() = default;
  Lease(const Lease&) = default;
private:
  Lease(Lease&&) = default;
};
Lease fresh() {
  Lease l;
  return l;
}
int main() {
  (void)fresh();
  return 0;
}
