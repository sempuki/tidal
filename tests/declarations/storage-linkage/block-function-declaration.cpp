// tidal-rules: LNK-9
// tidal-expect: run
// tidal-sources: block-function-declaration.tu.cpp
int main() {
  int defined_elsewhere();
  extern int counter;
  return defined_elsewhere() == 3 && counter == 4 ? 0 : 1;
}
