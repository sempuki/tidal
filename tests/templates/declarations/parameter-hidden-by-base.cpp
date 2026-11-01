// tidal-rules: TPL-41
// tidal-expect: run
// tidal-note: In the member definitions, B names the base class and Size names the member type. Both hide the parameter of the definition's head.
struct B { char pad[16]; };
template <class T> struct Holder : B {
  using Size = char[8];
  int base_size();
  int member_size();
};
template <class B> int Holder<B>::base_size() { return sizeof(B); }
template <class Size> int Holder<Size>::member_size() { return sizeof(Size); }
int main() { return Holder<char>{}.base_size() == 16 && Holder<char>{}.member_size() == 8 ? 0 : 1; }
