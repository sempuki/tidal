// tidal-rules: SEQ-10
// tidal-expect: run
int g = 0;
int seen(int arg) { return g * 10 + arg; }
int main() {
  int r = seen(g++);
  return r == 10 ? 0 : 1;
}
