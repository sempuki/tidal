// tidal-rules: RFOR-2
// tidal-expect: run
struct Stop {};
struct Iter {
  int v;
  int operator*() const { return v; }
  Iter& operator++() { ++v; return *this; }
  bool operator!=(Stop) const { return v < 3; }
};
struct Count { Iter begin() { return {0}; } Stop end() { return {}; } };
int main() {
  int sum = 0;
  for (int v : Count{}) sum += v;
  return sum == 3 ? 0 : 1;
}
