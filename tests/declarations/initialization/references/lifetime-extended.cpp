// tidal-rules: REF-4
// tidal-expect: run
char log[8];
int pos = 0;
struct Note {
  char c;
  ~Note() { log[pos++] = c; }
};
Note make(char c) { return Note{c}; }
int main() {
  {
    const Note& a = make('a');
    Note&& b = make('b');
    log[pos++] = '|';
  }
  make('t');
  log[pos++] = '|';
  return log[0] == '|' && log[1] == 'b' && log[2] == 'a' && log[3] == 't' && log[4] == '|' ? 0 : 1;
}
