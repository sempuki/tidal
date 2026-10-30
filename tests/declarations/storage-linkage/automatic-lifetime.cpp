// tidal-rules: STO-1
// tidal-expect: run
char log[8];
int pos = 0;
struct Note {
  char c;
  ~Note() { log[pos++] = c; }
};
const void* addresses[2];
void recurse(int depth) {
  int local = depth;
  addresses[depth] = &local;
  if (depth == 0) recurse(1);
}
int main() {
  {
    Note a{'a'};
    Note b{'b'};
  }
  recurse(0);
  return log[0] == 'b' && log[1] == 'a' && addresses[0] != addresses[1] ? 0 : 1;
}
