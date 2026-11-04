// tidal-rules: DEFARG-11
// tidal-expect: run
struct Key {
  int v;
  friend int open(Key k, int extra = 2) { return k.v + extra; }
};
int main() { return open(Key{1}) == 3 ? 0 : 1; }
