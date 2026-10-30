// tidal-rules: COND-2
// tidal-expect: run
int order[2];
int n = 0;
bool cond() { order[n++] = 1; return true; }
int val() { order[n++] = 2; return 5; }
int main() {
  int r = cond() ? val() : 0;
  return r == 5 && order[0] == 1 && order[1] == 2 ? 0 : 1;
}
