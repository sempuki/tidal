// tidal-rules: SEQ-2
// tidal-expect: run
char log[4];
int pos = 0;
struct S { int f(int v) { return v; } };
S s;
S& obj() { log[pos++] = 'o'; return s; }
int arg() { log[pos++] = 'a'; return 1; }
int main() {
  obj().f(arg());
  return log[0] == 'o' && log[1] == 'a' ? 0 : 1;
}
