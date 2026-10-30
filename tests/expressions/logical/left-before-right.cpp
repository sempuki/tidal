// tidal-rules: LOG-5
// tidal-expect: run
int main() {
  int i = 0;
  bool a = (i++ == 0) && (i == 1);
  bool b = (i++ == 5) || (i == 2);
  return a && b ? 0 : 1;
}
