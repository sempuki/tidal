// tidal-rules: UNW-1
// tidal-expect: run
int trace = 0;
void mark(int d) { trace = trace * 10 + d; }
struct Note { int id; ~Note() { mark(id); } };
struct Arg { int id; ~Arg() { mark(id); } };
int take(Arg) { return 0; }
int fail(const Note&) { throw 0; }
int main() {
  try {
    Note outer{5};
    (void)(take(Arg{1}), fail(Note{2}), mark(9));
  } catch (int) { mark(8); }
  // The parameter Arg{1} ends at the return of take or with the full
  // expression (VC-10), so both orders are valid.
  return trace == 2158 || trace == 1258 ? 0 : 1;
}
