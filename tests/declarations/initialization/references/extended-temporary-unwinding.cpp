// tidal-rules: REF-4
// tidal-expect: run
int ended = 0;
struct Note { int v; ~Note() { ++ended; } };
int main() {
  try {
    const Note& n = Note{1};
    (void)n;
    throw 1;
  } catch (int) {
    return ended == 1 ? 0 : 1;   // the extended temporary ended during unwinding
  }
}
