// The size of a UTF-8 string literal and its element type.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
int main() {
  static_assert(same<decltype(u8"x"[0]), const char8_t&>);
  return sizeof(u8"é") == 3 ? 0 : 1;
}
