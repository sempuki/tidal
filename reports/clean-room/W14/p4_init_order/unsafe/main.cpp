int seen_value();
// Exit 0 if the reader saw the final value, 1 if it saw zero.
int main() { return seen_value() == 100 ? 0 : 1; }
