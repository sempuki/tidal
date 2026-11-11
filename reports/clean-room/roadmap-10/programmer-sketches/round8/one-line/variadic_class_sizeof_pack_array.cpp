template <class... T> struct Names { static constexpr int count = sizeof...(T); int sizes[sizeof...(T)] = {sizeof(T)...}; };
int main() { Names<char, int> n; return n.count == 2 && n.sizes[1] == sizeof(int) ? 0 : 1; }
