// tidal-rules: INST-22
// tidal-expect: accept
template <class T> int get(T) { return 1; }
template <class T> auto deduce(T) { return 2; }
template <class T> int table[] = {1, 2, 3};
template int get(int);
template int get<>(long);
template int get<char>(char);
template auto deduce(int);
template int table<int>[3];
