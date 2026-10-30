// tidal-rules: SW-4
// tidal-expect: run
int main() {
  switch (int x = 3) { case 3: return x == 3 ? 0 : 1; }
  return 2;
}
