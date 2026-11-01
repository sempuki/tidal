// tidal-rules: TRY-18
// tidal-expect: run-fail
struct Error {
  Error() {}
  Error(const Error&) { throw 3; }
};
int main() {
  try {
    try { throw Error{}; }
    catch (Error e) { return 1; }
  } catch (int) { return 0; }
  return 0;
}
