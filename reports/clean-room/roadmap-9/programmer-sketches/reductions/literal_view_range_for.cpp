// A view class over a string literal with begin and end, in a range for.
struct View {
  const char* p;
  int n;
  constexpr const char* begin() const { return p; }
  constexpr const char* end() const { return p + n; }
};
constexpr int count_o(View v) {
  int k = 0;
  for (char c : v) k += c == 'o';
  return k;
}
static_assert(count_o(View{"world", 5}) == 1);
int main() { return count_o(View{"world", 5}) == 1 ? 0 : 1; }
