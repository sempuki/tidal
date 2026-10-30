// tidal-rules: LOG-11
// tidal-expect: run
// tidal-note: Gap review 2026-10-30.
int side = 0;
struct T { bool v; };
bool operator&&(T a, T b) { return a.v && b.v; }
T make(bool v) { ++side; return T{v}; }
int main() {
  bool r = T{false} && make(true);
  return !r && side == 1 ? 0 : 1;
}
