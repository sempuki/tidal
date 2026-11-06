using Callback = int (*)(int);
Callback slot;
bool add(Callback f) { slot = f; return true; }
extern bool done;
int main() { return done ? slot(0) : 1; }
