// tidal-rules: LNK-1
// tidal-expect: run
// tidal-sources: external-default.tu.cpp
int shared = 1;
int read_shared();
int main() {
  shared = 7;
  return read_shared() == 7 ? 0 : 1;
}
