// tidal-rules: DEL-1 RFOR-1
// tidal-expect: reject
// tidal-error-clang: call to deleted constructor
// tidal-error-gcc: use of deleted function
// tidal-error-edg: deleted function
// tidal-note: Roadmap round 9, programmer mistake copy_in_range_for_move_only.cpp. The loop variable has type Token, and each element initializes it (RFOR-1). An element is an lvalue, so the initialization selects the deleted copy constructor (DEL-1).
struct Token {
  int id;
  Token(int v) : id(v) {}
  Token(const Token&) = delete;
  Token(Token&&) = default;
};
int main() {
  Token row[2] = {Token(1), Token(2)};
  int sum = 0;
  for (auto t : row) sum += t.id;
  return sum == 3 ? 0 : 1;
}
