// tidal-rules: IF-2
// tidal-expect: run
int main() {
  int r = 0;
  if (true)
    if (false) r = 1;
    else r = 2;
  if (false)
    if (true) r = 3;
    else r = 4;
  return r == 2 ? 0 : 1;
}
