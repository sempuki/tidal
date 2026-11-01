// tidal-rules: CPY-8
// tidal-expect: run
// tidal-flags: -O0 -fsanitize=undefined -fno-sanitize-recover=undefined
// tidal-note: The implicit copy copies the pointer member, so the copy points into the source object. A copy constructor that the class writes points into the new object instead.
struct Plain {
  char text[4] = {'a', 'b', 'c', 0};
  char* cursor = text + 1;
};
struct Line {
  char text[4] = {'a', 'b', 'c', 0};
  char* cursor = text + 1;
  Line() = default;
  Line(const Line& other) : cursor(text + (other.cursor - other.text)) {
    for (int i = 0; i < 4; ++i) text[i] = other.text[i];
  }
};
int main() {
  Plain p;
  Plain q = p;
  bool plain_shares = q.cursor == p.text + 1;
  Line a;
  Line b = a;
  bool line_own = b.cursor == b.text + 1 && *b.cursor == 'b';
  return plain_shares && line_own ? 0 : 1;
}
