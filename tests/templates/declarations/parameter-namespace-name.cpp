// tidal-rules: TPL-41 TPL-19
// tidal-expect: run
// tidal-note: A class in the namespace of the class template does not hide the parameter B of the member definition.
namespace lib {
  struct B { char pad[16]; };
  template <class T> struct Holder { int size(); };
  template <class B> int Holder<B>::size() { return sizeof(B); }
}
int main() { return lib::Holder<char>{}.size() == 1 ? 0 : 1; }
