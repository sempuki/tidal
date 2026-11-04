// tidal-rules: VC-9
// tidal-expect: run
int trace = 0;
void mark(int d) { trace = trace * 10 + d; }
struct Note { int id; ~Note() { mark(id); } };
int use(const Note&) { return 7; }
int main() {
  int r = ([held = use(Note{3})] { return held; }(), mark(4), 0);
  return trace == 43 && r == 0 ? 0 : 1;
}
