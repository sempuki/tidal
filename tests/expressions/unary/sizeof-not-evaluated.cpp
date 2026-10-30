// tidal-rules: UN-13
// tidal-expect: run
int main() {
  int i = 0;
  auto n = sizeof(i++);
  return i == 0 && n == sizeof(int) ? 0 : 1;
}
