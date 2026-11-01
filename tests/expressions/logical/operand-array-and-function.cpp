// tidal-rules: LOG-2
// tidal-expect: run
int cells[2];
void act() {}
int main() {
  bool both = cells && act;
  bool none = !cells || !act;
  return both && !none ? 0 : 1;
}
