// tidal-rules: COMMA-1
// tidal-expect: run
int main() {
  int i = 0, j = 10;
  for (; i < 3; ++i, --j) {}
  int k = 0;
  if ((k = 4, k > 3)) k = 1;
  while ((++k, k < 5)) {}
  return i == 3 && j == 7 && k == 5 ? 0 : 1;
}
