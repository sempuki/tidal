// tidal-rules: LNK-3
// tidal-expect: run
// tidal-sources: const-internal.tu.cpp
const int limit = 1;
constexpr int depth = 10;
int other_limit();
int other_depth();
int main() {
  return limit == 1 && depth == 10 && other_limit() == 2 && other_depth() == 20 ? 0 : 1;
}
