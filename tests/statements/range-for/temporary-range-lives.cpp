// tidal-rules: RFOR-4
// tidal-expect: run
struct Holder {
  int data[3] = {1, 2, 3};
  int* begin() { return data; }
  int* end() { return data + 3; }
};
Holder make() { return {}; }
int main() {
  int sum = 0;
  for (int v : make()) sum += v;
  return sum == 6 ? 0 : 1;
}
