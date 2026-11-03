// tidal-rules: NAM-40
// tidal-expect: run
// tidal-note: A name in a list of bases finds the class, although the variable with the same name hides the class elsewhere.
struct node { int value = 4; };
int node = 3;
struct leaf : node { };
int main() {
  leaf l;
  return l.value + node == 7 ? 0 : 1;
}
