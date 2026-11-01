// tidal-rules: LOG-5
// tidal-expect: run
int trace[8];
int n = 0;
struct Probe {
  int id;
  Probe(int i) : id(i) {}
  ~Probe() { trace[n++] = -id; }
  bool check() const { trace[n++] = id; return true; }
};
int main() {
  bool r = Probe(1).check() && Probe(2).check();
  trace[n++] = 9;
  return r && n == 5 && trace[0] == 1 && trace[1] == 2 && trace[2] == -2 &&
         trace[3] == -1 && trace[4] == 9 ? 0 : 1;
}
