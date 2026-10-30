// tidal-rules: JMP-5
// tidal-expect: run
int main() {
  int value = 1;
  goto value;
value:
  return value == 1 ? 0 : 1;
}
