// tidal-rules: SEQ-5
// tidal-expect: run
int main() {
  int i = 0;
  int a = i++;
  int b = i;
  return a == 0 && b == 1 ? 0 : 1;
}
