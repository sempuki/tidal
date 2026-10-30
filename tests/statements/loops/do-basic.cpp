// tidal-rules: LOOP-2
// tidal-expect: run
int main() {
  int runs = 0;
  do { ++runs; } while (false);
  return runs == 1 ? 0 : 1;
}
