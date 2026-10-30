// tidal-rules: FN-10
// tidal-expect: run
// tidal-note: Gap review 2026-10-30.
char log[4];
int pos = 0;
struct Note { char c; ~Note() { log[pos++] = c; } };
int take(Note) { return 0; }
void mark() { log[pos++] = 'm'; }
int main() {
  (take(Note{'p'}), mark());
  bool early = log[0] == 'p' && log[1] == 'm';
  bool late = log[0] == 'm' && log[1] == 'p';
  return early || late ? 0 : 1;
}
