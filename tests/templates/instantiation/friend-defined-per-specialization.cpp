// tidal-rules: INST-13
// tidal-expect: run
template <class T> struct Box { friend int hook(Box) { return sizeof(T); } };
int main() { return hook(Box<char>{}) == 1 && hook(Box<int>{}) == sizeof(int) ? 0 : 1; }
