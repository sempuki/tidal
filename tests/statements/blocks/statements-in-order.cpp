// tidal-rules: STMT-3
// tidal-expect: run
int order[3];
int n = 0;
int main() {
  order[n++] = 1;
  { order[n++] = 2; }
  order[n++] = 3;
  return order[0] == 1 && order[1] == 2 && order[2] == 3 ? 0 : 1;
}
