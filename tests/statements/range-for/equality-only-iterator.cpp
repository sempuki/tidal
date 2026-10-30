// tidal-rules: RFOR-2
// tidal-expect: run
// tidal-note: Gap review 2026-10-30.
struct It {
  int v;
  int operator*() const { return v; }
  It& operator++() { ++v; return *this; }
  bool operator==(const It& o) const { return v == o.v; }
};
struct R { It begin() { return {0}; } It end() { return {3}; } };
int main() {
  int sum = 0;
  for (int x : R{}) sum += x;
  return sum == 3 ? 0 : 1;
}
