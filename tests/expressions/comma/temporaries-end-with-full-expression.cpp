// tidal-rules: COMMA-1
// tidal-expect: run
int trace[8];
int n = 0;
struct Probe {
  int id;
  Probe(int i) : id(i) { trace[n++] = id; }
  ~Probe() { trace[n++] = -id; }
};
int main() {
  int r = (Probe(1), Probe(2), 5);
  trace[n++] = 9;
  return r == 5 && n == 5 && trace[0] == 1 && trace[1] == 2 && trace[2] == -2 &&
         trace[3] == -1 && trace[4] == 9 ? 0 : 1;
}
