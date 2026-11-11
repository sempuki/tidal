template <class T> concept Int = sizeof(T) == sizeof(int);
int main() { Int auto x = 3; return x - 3; }
