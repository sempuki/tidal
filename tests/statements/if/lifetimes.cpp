// tidal-rules: IF-11
// tidal-expect: run
char log[16];
int pos = 0;
struct Note {
  char c;
  ~Note() { log[pos++] = c; }
  explicit operator bool() const { return true; }
};
int main() {
  {
    if (Note a{'a'}; Note{'x'}) {
      log[pos++] = 'B';
    }
  }
  {
    if (Note a{'a'}; Note c{'c'}) {
      log[pos++] = 'B';
    } else {
      log[pos++] = 'E';
    }
  }
  log[pos] = 0;
  const char* want = "xBaBca";
  for (int i = 0; want[i] || log[i]; ++i)
    if (want[i] != log[i]) return 1;
  return 0;
}
