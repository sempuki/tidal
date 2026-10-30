// tidal-rules: JMP-5
// tidal-expect: run
int main() {
  int n = 0;
again:
  ++n;
  if (n < 3) goto again;
  goto done;
  n = 100;
done:
  return n == 3 ? 0 : 1;
}
