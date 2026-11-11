// A pointer to data member as a run-time key for a sort.
struct Rec { int id; int score; };
void sort_by(Rec* r, int n, int Rec::*key) {
  for (int i = 1; i < n; ++i) for (int j = i; j > 0 && r[j].*key < r[j - 1].*key; --j) { Rec t = r[j]; r[j] = r[j - 1]; r[j - 1] = t; }
}
int main() { Rec r[] = {{1, 30}, {2, 10}, {3, 20}}; sort_by(r, 3, &Rec::score); return r[0].id == 2 && r[2].id == 1 ? 0 : 1; }
