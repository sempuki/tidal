// if consteval picks a different branch during constant evaluation.
constexpr int where() {
  if consteval { return 1; } else { return 2; }
}
int main() {
  constexpr int c = where();
  int r = where();
  return c == 1 && r == 2 ? 0 : 1;
}
