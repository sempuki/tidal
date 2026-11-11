struct Box { int* p = nullptr; Box() = default; Box(const Box&) = delete; Box(Box&&) = default; };
int main() { Box a; Box b = a; (void)b; }
