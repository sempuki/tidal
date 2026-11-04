// tidal-rules: VC-9
// tidal-expect: run
int trace = 0;
void mark(int d) { trace = trace * 10 + d; }
struct Note { int id; ~Note() { mark(id); } };
int use(const Note&) { return 0; }
int next() { mark(5); return 0; }
struct Parts {
  int a = use(Note{1});
  int b = next();
  int c;
  int d;
  Parts() : c(use(Note{2})), d(next()) {}
};
int main() {
  Parts p;
  return trace == 1525 ? 0 : 1;
}
