int main() {
  char* p = const_cast<char*>("text");
  p[0] = 'T';
  return p[0] == 'T' ? 0 : 1;
}
