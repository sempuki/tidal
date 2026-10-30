// tidal-rules: RFOR-3
// tidal-expect: run
// tidal-note: Gap review 2026-10-30.
struct Grow {
  int data[8] = {1, 1, 1, 1, 1, 1, 1, 1};
  int size = 2;
  int* begin() { return data; }
  int* end() { return data + size; }
};
int main() {
  Grow g;
  int seen = 0;
  for (int x : g) { ++g.size; seen += x; }
  return seen == 2 ? 0 : 1;
}
