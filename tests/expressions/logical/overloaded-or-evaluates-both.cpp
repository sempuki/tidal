// tidal-rules: LOG-11
// tidal-expect: run
int side = 0;
struct T { bool v; };
bool operator||(T a, T b) { return a.v || b.v; }
T make(bool v) { ++side; return T{v}; }
int main() {
  bool r = T{true} || make(false);
  return r && side == 1 ? 0 : 1;
}
