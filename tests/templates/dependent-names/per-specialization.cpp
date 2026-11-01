// tidal-rules: DEP-7
// tidal-expect: run
struct Small { static constexpr int value = 1; };
struct Large { static constexpr int value = 2; };
template <class T> int read() { return T::value; }
int main() { return read<Small>() == 1 && read<Large>() == 2 ? 0 : 1; }
