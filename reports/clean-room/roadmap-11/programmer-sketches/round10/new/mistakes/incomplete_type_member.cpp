// Planted mistake: a data member of a class that is only declared.
struct Engine;
struct Car { Engine e; };
int main() { return 0; }
