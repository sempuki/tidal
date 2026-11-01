// tidal-rules: STMT-1 STMT-3
// tidal-expect: run
int main() {
  int n = 0;
  {}
  { { ; } }
  { ++n; { ++n; } }
  ;;
  return n == 2 ? 0 : 1;
}
