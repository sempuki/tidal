// tidal-rules: FN-12
// tidal-expect: reject
// tidal-error-clang: incomplete result type .Later
// tidal-error-gcc: return type .struct Later. is incomplete
// tidal-error-edg: returns incomplete type .Later
// tidal-note: P4-1. User defines make in its class with the return type Later. Later is incomplete at the end of User and is completed only after it, so the definition of make breaks FN-12. GCC, Clang, and EDG reject.
struct Later;
struct User {
  Later make() const { throw 0; }
};
struct Later { int k; };
int main() {}
