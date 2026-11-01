// tidal-rules: SEQ-6
// tidal-expect: run
char log_[4]; int n = 0;
struct Field {};
struct Record { int operator->*(Field) { return 6; } };
Record rec;
Record& object() { log_[n++] = 'o'; return rec; }
Field field() { log_[n++] = 'f'; return {}; }
int main() {
  int v = object()->*field();
  return v == 6 && log_[0] == 'o' && log_[1] == 'f' ? 0 : 1;
}
