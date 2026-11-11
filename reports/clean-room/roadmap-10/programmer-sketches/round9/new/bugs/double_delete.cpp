struct Owner { int* p; ~Owner() { delete p; } };
int main() { Owner a{new int(1)}; Owner b = a; return *b.p - 1; }
