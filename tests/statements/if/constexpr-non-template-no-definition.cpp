// tidal-rules: IF-15
// tidal-expect: run
int never_defined();
int main() {
  if constexpr (false) {
    return never_defined();
  }
  return 0;
}
