// tidal-rules: STG-10 UNI-14
// tidal-expect: run
// tidal-note: W20-2, W20 reduction union-array-member-subscript.cpp. Q-0425. The union Store has no active member. The program forms pointers to elements 0 and 1 of the array member cells and compares them. No object is read. Every compiler builds and runs the program, and the test follows them while the question is open.
union Store {
  int cells[3];
  char flag;
  Store() {}
};
int main() {
  Store s;
  int* first = &s.cells[0];
  int* second = &s.cells[1];
  return (first != nullptr && first != second) ? 0 : 1;
}
