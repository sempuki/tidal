// tidal-rules: LOOP-3
// tidal-expect: run
char log[16];
int pos = 0;
bool cond(int i) { log[pos++] = 'c'; return i < 2; }
int main() {
  for (log[pos++] = 'i', 0; cond(pos); log[pos++] = 's') log[pos++] = 'b';
  return log[0] == 'i' && log[1] == 'c' ? 0 : 1;
}
