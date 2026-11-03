// tidal-rules: TRY-17
// tidal-expect: reject
// tidal-error-clang: return in the catch of a function try block of a constructor
// tidal-error-gcc: cannot return from a handler of a function-try-block of a constructor
// tidal-error-edg: return statement is not allowed in a handler of a function try block of a constructor
// tidal-note: The return statement is in a nested block of the handler, and the constructor has a member initializer.
struct S {
  int m;
  S(int v) try : m(v) {
  } catch (...) {
    if (true) {
      return;
    }
  }
};
