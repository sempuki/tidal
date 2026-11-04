// tidal-rules: TRY-11
// tidal-expect: run-fail
// tidal-flags: -O0 -fsanitize=vptr -fno-sanitize-recover=all
// tidal-expect-clang: run
// tidal-error-gcc: member call on address .* which does not point to an object of type
// tidal-note: part has ended when the handler of the constructor's function try block runs, so the call is undefined. The Clang sanitizer does not check this call.
// tidal-note: Run-time check with the sanitizer (decision 0028).
struct Part {
  virtual ~Part() {}
  int value() const { return 1; }
};
struct Whole {
  Part part;
  Whole() try { throw 1; } catch (int) {
    int v = part.value();   // undefined: part has ended
    (void)v;
  }
};
int main() {
  try {
    Whole whole;
  } catch (int) {
    return 0;
  }
  return 1;
}
