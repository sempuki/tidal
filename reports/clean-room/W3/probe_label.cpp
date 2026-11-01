int main() {
  int hits = 0;
outer:
  for (int i = 0; i < 3; ++i) {
    for (int j = 0; j < 3; ++j) {
      if (i * j == 2) break outer;
      ++hits;
    }
  }
  return hits == 5 ? 0 : 1;
}
