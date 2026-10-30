// tidal-rules: SB-6
// tidal-expect: run
char log[4];
int pos = 0;
struct Note { char c; ~Note() { log[pos++] = c; } };
struct Two { Note a; Note b; };
int main() {
  {
    const auto& [x, y] = Two{Note{'a'}, Note{'b'}};
    log[pos++] = '|';
    (void)x;
  }
  return log[0] == '|' ? 0 : 1;
}
