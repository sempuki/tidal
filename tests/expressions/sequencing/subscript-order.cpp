// tidal-rules: SEQ-3
// tidal-expect: run
char log[4];
int pos = 0;
int data[3] = {10, 20, 30};
int* base() { log[pos++] = 'b'; return data; }
int index() { log[pos++] = 'i'; return 1; }
int main() {
  int v = base()[index()];
  return v == 20 && log[0] == 'b' && log[1] == 'i' ? 0 : 1;
}
