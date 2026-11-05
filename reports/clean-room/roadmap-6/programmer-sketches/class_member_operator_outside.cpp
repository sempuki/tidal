struct Fixed {
  long long raw;
  bool operator<(Fixed o) const;
};
bool Fixed::operator<(Fixed o) const { return raw < o.raw; }
int main() {
  Fixed a{3}, b{5};
  return a < b ? 0 : 1;
}
