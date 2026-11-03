// tidal-rules: CX-2
// tidal-expect: accept
// tidal-note: Write report W11. A string literal is an object with static storage duration, so a constexpr pointer can refer to it, also in a block.
constexpr const char* names[3] = {"red", "green", "blue"};
static_assert(names[1][0] == 'g');
int use() {
  constexpr const char* local = "box";
  return local[0];
}
