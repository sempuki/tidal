// tidal-rules: STO-10
// tidal-expect: run
// tidal-note: Gap review 2026-10-30.
char log[4];
int pos = 0;
struct Note {
  char c;
  ~Note() {
    log[pos++] = c;
    if (c == 'a' && !(log[0] == 'c' && log[1] == 'b' && log[2] == 'a')) throw 1;
  }
};
Note a{'a'};
Note b{'b'};
void reach() { static Note c{'c'}; }
int main() { reach(); return 0; }
