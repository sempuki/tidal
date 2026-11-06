// ICS-46: two direct bindings to int[2]; the known bound wins.
int row[2];
int take(int (&)[2]) { return 1; }
int take(int (&)[]) { return 2; }
int main() { return take(row) == 1 ? 0 : 1; }
