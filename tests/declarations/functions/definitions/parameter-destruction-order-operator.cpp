// tidal-rules: FN-10
// tidal-expect: run
// tidal-note: The parameter belongs to an overloaded operator. Either time of destruction is allowed.
char log[4];
int pos = 0;
struct Note { char c; ~Note() { log[pos++] = c; } };
int operator+(Note, int v) { return v; }
void mark() { log[pos++] = 'm'; }
int main() {
  (Note{'p'} + 1, mark());
  bool early = log[0] == 'p' && log[1] == 'm';
  bool late = log[0] == 'm' && log[1] == 'p';
  return early || late ? 0 : 1;
}
