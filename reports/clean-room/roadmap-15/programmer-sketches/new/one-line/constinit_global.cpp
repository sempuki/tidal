constexpr int square(int x) { return x * x; }
constinit int table_size = square(4);
int main() {
  table_size += 1;
  return table_size == 17 ? 0 : 1;
}
