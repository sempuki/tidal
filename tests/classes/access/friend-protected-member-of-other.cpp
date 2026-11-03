// tidal-rules: ACC-15
// tidal-expect: reject
// tidal-error-clang: friend function .* is a protected member
// tidal-error-gcc: protected within this context
// tidal-error-edg: is inaccessible
// tidal-note: The friend declaration names a protected member function of an unrelated class.
class Other {
protected:
  void hidden();
};
class C {
  friend void Other::hidden();
};
