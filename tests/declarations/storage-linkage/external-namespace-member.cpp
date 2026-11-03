// tidal-rules: LNK-1
// tidal-expect: run
// tidal-sources: external-namespace-member.tu.cpp
// tidal-note: A variable and a function in a named namespace. Both are defined in the other unit, and this unit only declares them.
namespace config {
extern int level;
int bump(int by);
}
int main() {
  config::level = 3;
  return config::bump(2) == 5 && config::level == 5 ? 0 : 1;
}
