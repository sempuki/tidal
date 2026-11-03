// tidal-rules: STO-10
// tidal-expect: run
// tidal-note: Two block-scope static variables, reached in the reverse order of their functions. They are destroyed in the reverse order of their completed initialization, before the namespace-scope variable. A destructor that finds a wrong order throws, so the program ends abnormally.
char log[4];
int pos = 0;
struct Note {
  char c;
  ~Note() {
    log[pos++] = c;
    if (c == 'g' && !(log[0] == 'x' && log[1] == 'y' && log[2] == 'g')) throw 1;
  }
};
Note g{'g'};
void first() { static Note x{'x'}; }
void second() { static Note y{'y'}; }
int main() {
  second();
  first();
  return 0;
}
