int classify(int x) {
  if (x > 0) [[likely]] return 1;
  else [[unlikely]] return -1;
}
int main() { return classify(3) + classify(-2); }
