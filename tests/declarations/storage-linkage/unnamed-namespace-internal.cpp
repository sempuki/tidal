// tidal-rules: LNK-5
// tidal-expect: run
// tidal-sources: unnamed-namespace-internal.tu.cpp
namespace {
int value = 1;
int helper() { return 10; }
}
int other_value();
int other_helper();
int main() {
  return value == 1 && helper() == 10 && other_value() == 2 && other_helper() == 20 ? 0 : 1;
}
