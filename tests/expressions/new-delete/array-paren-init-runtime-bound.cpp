// tidal-rules: NEW-2
// tidal-expect: run
int main(int argc, char**) {
  int rows = argc + 1, columns = 3;
  int* cells = new int[rows * columns]();   // each element is 0
  int sum = 0;
  for (int i = 0; i != rows * columns; ++i) sum += cells[i];
  delete[] cells;
  return sum == 0 ? 0 : 1;
}
