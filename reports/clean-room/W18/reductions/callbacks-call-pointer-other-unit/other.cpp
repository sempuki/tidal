using Callback = int (*)(int);
bool add(Callback f);
int id(int x) { return x; }
bool done = add(id);
