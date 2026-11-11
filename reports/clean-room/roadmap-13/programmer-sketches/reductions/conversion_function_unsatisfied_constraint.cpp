template <bool B> struct Q {
  int value = 0;
  constexpr explicit operator int() const requires B { return value; }
};
template <class X> concept HasNumber = requires(X q) { static_cast<int>(q); };
static_assert(HasNumber<Q<true>>);
static_assert(!HasNumber<Q<false>>);
int main() { return 0; }
