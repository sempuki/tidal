// tidal-rules: CPY-4 CPY-3
// tidal-expect: run
// tidal-note: The user-declared move assignment makes the implicit copy assignment deleted (CPY-4). The prvalue on the right selects the move assignment.
struct Token {
  int id;
  Token(int v) : id(v) {}
  Token& operator=(Token&& from) { id = from.id; from.id = 0; return *this; }
};
int main() {
  Token t(1);
  t = Token(5);
  return t.id == 5 ? 0 : 1;
}
