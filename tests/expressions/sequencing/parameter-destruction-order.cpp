// tidal-rules: VC-15
// tidal-expect: run
int order[4];
int n = 0;
struct Note {
  int id;
  Note(int i) : id(i) {}
  Note(const Note& o) : id(o.id) {}
  ~Note() { order[n++] = id; }
};
void take(Note, Note) {}
int main() {
  take(Note(1), Note(2));
  bool either = (order[0] == 1 && order[1] == 2) || (order[0] == 2 && order[1] == 1);
  return n == 2 && either ? 0 : 1;
}
