// tidal-rules: CE-30 CE-8
// tidal-expect: reject
// tidal-error-gcc: called in a constant expression|not a constant expression
// tidal-error-clang: must be initialized by a constant expression
// tidal-note: The constexpr variable starts its own evaluation. The try block around it is not part of that evaluation.
constexpr int fail() { throw 1; return 0; }
constexpr int f() {
  try {
    constexpr int x = fail();
    return x;
  } catch (...) {
    return 2;
  }
}
int main() { return f(); }
