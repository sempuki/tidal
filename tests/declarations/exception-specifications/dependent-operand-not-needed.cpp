// tidal-rules: EXS-9
// tidal-expect: accept
// int has no member safe, but no use needs the specifier of these
// specializations.
template <class T> struct Box { void put() noexcept(T::safe); };
Box<int> box;
template <class T> void store(T) noexcept(T::safe);
void store(int) {}
int main() { store(1); }
