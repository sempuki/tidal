// tidal-rules: STO-7
// tidal-expect: run
int shared = 5;
int read() {
  int shared = 1;
  {
    extern int shared;
    return shared;
  }
}
int main() { return read() == 5 ? 0 : 1; }
