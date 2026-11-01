// tidal-rules: LIST-7
// tidal-expect: run
int order[5];
int n = 0;
int note(int v) { order[n++] = v; return v; }
struct S { int a; int b; int c; };
int main() {
  S s{note(1), note(2), note(3)};
  int arr[]{note(4), note(5)};
  bool in_order = order[0] == 1 && order[1] == 2 && order[2] == 3 && order[3] == 4 && order[4] == 5;
  return in_order && n == 5 && s.c == 3 && arr[1] == 5 ? 0 : 1;
}
