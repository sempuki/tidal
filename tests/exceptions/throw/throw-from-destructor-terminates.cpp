// tidal-rules: THR-9
// tidal-expect: run-fail
// tidal-note: A destructor without a written specifier is non-throwing. An exception that leaves it ends the program, and the caller's handler does not catch it.
struct Bad {
  ~Bad() { throw 1; }
};
int main() {
  try {
    Bad b;
  } catch (...) {
    return 0;
  }
  return 0;
}
