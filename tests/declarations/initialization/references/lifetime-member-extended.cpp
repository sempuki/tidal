// tidal-rules: REF-4
// tidal-expect: run
char log[8];
int pos = 0;
struct Note {
  char c;
  ~Note() { log[pos++] = c; }
};
struct Pair { Note first; Note second; };
int main() {
  {
    const Note& n = Pair{Note{'x'}, Note{'y'}}.second;
    log[pos++] = '|';
    (void)n;
  }
  return log[0] == '|' ? 0 : 1;
}
