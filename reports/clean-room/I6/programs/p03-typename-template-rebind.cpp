// DEP-19, DEP-24: typename T::template rebind<U>::other.
struct Alloc {
  template <class U> struct rebind { using other = U; };
};
template <class T, class U> int width() {
  using other = typename T::template rebind<U>::other;
  return sizeof(other);
}
int main() { return width<Alloc, char>() == 1 && width<Alloc, int>() == sizeof(int) ? 0 : 1; }
