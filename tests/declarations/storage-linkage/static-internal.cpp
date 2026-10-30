// tidal-rules: LNK-2
// tidal-expect: run
// tidal-sources: static-internal.tu.cpp
static int value = 1;
static int helper() { return 10; }
int other_value();
int other_helper();
int main() {
  return value == 1 && helper() == 10 && other_value() == 2 && other_helper() == 20 ? 0 : 1;
}
