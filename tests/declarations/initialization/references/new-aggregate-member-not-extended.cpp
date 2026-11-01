// tidal-rules: REF-5
// tidal-expect: run
int trace = 0;
void mark(int d) { trace = trace * 10 + d; }
struct Note { int id; ~Note() { mark(id); } };
struct Holder { const Note& note; };
int main() {
  Holder* braced = new Holder{Note{1}};
  mark(2);
  delete braced;
  Holder* parens = new Holder(Note{3});
  mark(4);
  delete parens;
  return trace == 1234 ? 0 : 1;
}
