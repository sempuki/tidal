// tidal-rules: SB-6
// tidal-expect: run
// tidal-note: The hidden variable is an rvalue reference. The temporary lives until the end of the block.
char log[4];
int pos = 0;
struct Note { char c; ~Note() { log[pos++] = c; } };
struct Two { Note a; Note b; };
int main() {
  {
    auto&& [x, y] = Two{Note{'a'}, Note{'b'}};
    x.c = 'x';
    log[pos++] = '|';
  }
  return log[0] == '|' && pos == 3 ? 0 : 1;
}
