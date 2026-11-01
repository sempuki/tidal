// tidal-rules: RFOR-3
// tidal-expect: run
struct Range {
  int data[2] = {1, 2};
  Range() = default;
  Range(Range&&) = delete;
  int* begin() & { return data; }
  int* end() & { return data + 2; }
};
int main() {
  int sum = 0;
  for (int v : Range{}) sum += v;
  Range named;
  for (int& v : static_cast<Range&&>(named)) v *= 10;
  return sum == 3 && named.data[1] == 20 ? 0 : 1;
}
