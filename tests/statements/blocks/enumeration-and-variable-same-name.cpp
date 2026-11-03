// tidal-rules: STMT-9
// tidal-expect: run
// tidal-note: The block declares an enumeration and a variable with the same name. The variable hides the enumeration, which the program names with enum.
int main() {
  enum Mode { on = 2 };
  int Mode = 5;
  enum Mode m = on;
  return Mode + m == 7 ? 0 : 1;
}
