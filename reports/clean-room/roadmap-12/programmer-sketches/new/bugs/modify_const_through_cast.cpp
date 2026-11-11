// Planted bug: a const object modified through const_cast.
const int limit = 10;
void raise(const int& r) { const_cast<int&>(r) = 20; }
int main() { raise(limit); return 0; }
