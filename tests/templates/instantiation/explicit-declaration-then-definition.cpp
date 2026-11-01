// tidal-rules: INST-26 INST-27
// tidal-expect: run
template <class T> int get(T) { return 1; }
extern template int get(int);
extern template int get(int);
int use() { return get(0); }
template int get(int);
int main() { return use() == 1 ? 0 : 1; }
