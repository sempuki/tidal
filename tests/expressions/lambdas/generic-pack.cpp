// tidal-rules: LAM-15 PACK-1
// tidal-expect: run
// tidal-note: Roadmap round 7, programmer reduction lambda_auto_pack.cpp (reports/clean-room/roadmap-7/programmer.md, section 1.5). Parameters auto... make the call operator a template with a pack.
int main() {
  auto sum = [](auto... a) { return (a + ... + 0); };
  return sum(1, -1) == 0 && sum() == 0 && sum(1, 2L, 3) == 6 ? 0 : 1;
}
