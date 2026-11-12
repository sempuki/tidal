struct Holder { using type = int; };
template <class T> T::type make() { return 4; }
template <class T> using value_t = T::type;
int main() { value_t<Holder> v = make<Holder>(); return v - 4; }
