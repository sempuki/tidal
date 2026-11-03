// tidal-rules: LNK-5
// tidal-expect: run
// tidal-sources: unnamed-namespace-nested.tu.cpp
// tidal-note: The unnamed namespace is inside a named namespace. Each unit defines its own tools::limit and tools::scale.
namespace tools {
namespace {
int limit = 100;
int scale(int v) { return v * 2; }
}
}
int other_limit();
int other_scale(int);
int main() {
  return tools::limit == 100 && tools::scale(3) == 6 && other_limit() == 200 && other_scale(3) == 9 ? 0 : 1;
}
