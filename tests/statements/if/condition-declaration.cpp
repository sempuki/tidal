// tidal-rules: IF-5 IF-9
// tidal-expect: run
int next(int v) { return v; }
int main() {
  int seen = -1;
  if (int x = next(0)) seen = 100;
  else seen = x;
  if (int y{next(7)}) seen += y;
  if (auto p = &seen) seen += (*p == 7 ? 1 : 0);
  return seen == 8 ? 0 : 1;
}
