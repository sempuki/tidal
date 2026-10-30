// tidal-rules: ATTR-8
// tidal-expect: run
int classify(int v) {
  if (v > 0) [[likely]] {
    return 1;
  } else [[unlikely]] {
    return -1;
  }
}
int main() {
  switch (classify(5)) {
    [[likely]] case 1: return classify(-5) == -1 ? 0 : 2;
    [[unlikely]] default: return 3;
  }
}
