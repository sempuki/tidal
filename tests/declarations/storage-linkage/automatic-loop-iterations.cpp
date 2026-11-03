// tidal-rules: STO-1
// tidal-expect: run
// tidal-note: A variable in the body of a loop. Each iteration creates the object and destroys it before the next iteration starts.
char log[8];
int pos = 0;
struct Note {
  Note() { log[pos++] = '+'; }
  ~Note() { log[pos++] = '-'; }
};
int main() {
  for (int i = 0; i < 3; ++i) {
    Note n;
  }
  for (int i = 0; i < 6; ++i)
    if (log[i] != (i % 2 == 0 ? '+' : '-')) return 1;
  return pos == 6 ? 0 : 2;
}
