// tidal-rules: CMP-26
// tidal-expect: run
int grid[4][5];
volatile int row = 2;
int main() {
  int* end = grid[row] + 5;
  int* next = grid[row + 1];
  return end == next && !(end < next) ? 0 : 1;
}
