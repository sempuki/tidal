// tidal-rules: IF-15
// tidal-expect: run
// tidal-note: A variable with no definition, used only in the discarded else branch.
extern int never_defined;
int main() {
  if constexpr (true) {
    return 0;
  } else {
    return never_defined;
  }
}
