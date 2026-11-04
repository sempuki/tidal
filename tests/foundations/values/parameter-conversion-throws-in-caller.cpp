// tidal-rules: VC-16
// tidal-expect: run
// tidal-note: The parameter is initialized by a converting constructor that throws. The exception leaves the call in the caller, so the noexcept function does not end the program.
struct Conv {
  Conv(int v) { throw v; }
};
void take(Conv) noexcept {}
int main() {
  try {
    take(3);
  } catch (int v) {
    return v == 3 ? 0 : 1;
  }
  return 2;
}
