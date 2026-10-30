// tidal-rules: NEW-9
// tidal-expect: run
char log[4];
int pos = 0;
struct S { char c; ~S() { log[pos++] = c; } };
int main() {
  S* a = new S[3]{{'a'}, {'b'}, {'c'}};
  delete[] a;
  return log[0] == 'c' && log[1] == 'b' && log[2] == 'a' ? 0 : 1;
}
