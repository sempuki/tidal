// tidal-rules: RFOR-2
// tidal-expect: run
struct Range {
  int data[3] = {4, 5, 6};
  int* begin() { return data; }
  int* end() { return data + 3; }
};
int main() {
  Range r;
  int sum = 0;
  for (int v : r) sum += v;
  return sum == 15 ? 0 : 1;
}
