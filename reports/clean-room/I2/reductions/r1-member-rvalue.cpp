struct X { template <class T> int operator+(T); };
template <class T> int operator+(X&&, T);
int use() { return X{} + 1; }
