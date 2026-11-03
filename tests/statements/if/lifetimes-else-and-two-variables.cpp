// tidal-rules: IF-11
// tidal-expect: run
// tidal-note: The temporary of the condition is destroyed before the else branch runs. Two variables of one init-statement are destroyed in reverse order when the if statement ends.
char log[16];
int pos = 0;
struct Note {
  char c;
  ~Note() { log[pos++] = c; }
  bool no() const { return false; }
};
int main() {
  if (Note{'x'}.no()) {
    log[pos++] = 'B';
  } else {
    log[pos++] = 'E';
  }
  if (Note a{'a'}, b{'b'}; true) {
    log[pos++] = 'B';
  }
  log[pos] = 0;
  const char* want = "xEBba";
  for (int i = 0; want[i] || log[i]; ++i)
    if (want[i] != log[i]) return 1;
  return 0;
}
