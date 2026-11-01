// tidal-rules: COMMA-7
// tidal-expect: run
int used = 0;
struct Tag {};
int operator,(Tag, int) { used = 1; return 7; }
void nothing() {}
int main() {
  (Tag{}, nothing());
  int before = used;
  int a = (Tag{}, 2);
  return before == 0 && a == 7 && used == 1 ? 0 : 1;
}
