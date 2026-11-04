// tidal-rules: VC-15
// tidal-expect: run
// tidal-note: Three parameters. Any order of destruction is allowed, and each parameter is destroyed once.
int order[4];
int n = 0;
struct Note {
  int id;
  Note(int i) : id(i) {}
  ~Note() { order[n++] = id; }
};
void take(Note, Note, Note) {}
int main() {
  take(Note(1), Note(2), Note(3));
  int seen = 0;
  for (int i = 0; i < n; ++i) seen |= 1 << order[i];
  return n == 3 && seen == 14 ? 0 : 1;
}
