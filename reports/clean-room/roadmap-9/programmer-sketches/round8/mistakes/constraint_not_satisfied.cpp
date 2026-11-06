template <class T> concept Small = sizeof(T) <= 4;
void f(Small auto) {}
int main() { f(1.0); }
