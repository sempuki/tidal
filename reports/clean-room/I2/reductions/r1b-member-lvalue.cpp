struct X { template <class T> int operator+(T); };
template <class T> int operator+(X&, T);
X x;
int use() { return x + 1; }
