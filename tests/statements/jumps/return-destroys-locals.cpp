// tidal-rules: JMP-4
// tidal-expect: run
char log[4];
int pos = 0;
struct Note { char c; ~Note() { log[pos++] = c; } };
int f() {
  Note a{'a'};
  Note b{'b'};
  return pos;
}
int main() { int seen = f(); return seen == 0 && log[0] == 'b' && log[1] == 'a' ? 0 : 1; }
