// tidal-rules: FN-12
// tidal-expect: reject
// tidal-error-clang: incomplete result type .Later
// tidal-error-gcc: return type .struct Later. is incomplete
// tidal-error-edg: returns incomplete type .Later
// tidal-note: RV9-1. User defines the friend function make in its class with the return type Later. Later is incomplete where User ends and is completed only after it, so the definition breaks FN-12. GCC, Clang, and EDG reject.
struct Later;
struct User {
  friend Later make(User) { throw 0; }
};
struct Later { int k; };
int main() {}
