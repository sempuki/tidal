// tidal-rules: VC-8
// tidal-expect: run
int made = 0;
int gone = 0;
struct Counted {
  Counted() { ++made; }
  Counted(const Counted&) { ++made; }
  ~Counted() { ++gone; }
  int v = 4;
};
int main() {
  bool ok = false;
  {
    Counted&& r = Counted();
    const Counted& s = Counted();
    int&& i = 5;
    i = 6;
    r.v = 9;
    ok = made == 2 && gone == 0 && i == 6 && r.v == 9 && s.v == 4;
  }
  return ok && gone == 2 ? 0 : 1;
}
