// tidal-rules: NEW-12 NEW-18
// tidal-expect: run
int order[4];
int n = 0;
int mark(int v) { order[n++] = v; return v; }
void* operator new[](decltype(sizeof 0) size, int) {
  mark(3);
  return ::operator new[](size);
}
int main() {
  int* p = new (mark(1)) int[mark(2)];
  bool either = (order[0] == 1 && order[1] == 2) || (order[0] == 2 && order[1] == 1);
  delete[] p;
  return either && order[2] == 3 ? 0 : 1;
}
