#include <array>
#include <cassert>
#include <cmath>
#include <concepts>
#include <limits>
#include <print>
#include <utility>

template <std::floating_point T>
void solution() {
  T eps = 1;
  int M_bits = 0, max_exp = 0, min_exp = 1;

  while (1 + eps / 2 != 1) {
    eps /= 2;
    M_bits++;
  }

  T value = 1;
  while (std::isfinite(value)) {
    value *= 2;
    max_exp++;
  }

  value = 1;
  while (std::isnormal(value)) {
    value /= 2;
    min_exp--;
  }
  min_exp++;

  std::println("epsillon: {}", eps);
  assert(eps == std::numeric_limits<T>::epsilon());
  std::println("mantissa size: {}", M_bits);
  assert(M_bits == std::numeric_limits<T>::digits - 1);
  std::println("max exp: {}", max_exp);
  assert(max_exp == std::numeric_limits<T>::max_exponent);
  std::println("min exp: {}", min_exp);
  assert(min_exp == std::numeric_limits<T>::min_exponent);

  std::array<std::pair<const char*, T>, 4> arr(
      {{"1", 1},
       {"1 + eps/2", 1 + eps / 2},
       {"1 + eps", 1 + eps},
       {"1 + eps + eps/2", 1 + eps + eps / 2}});

  for (int i = 0; i < arr.size(); i++) {
    for (int j = i; j < arr.size(); j++) {
      const auto& [n1, item1] = arr[i];
      const auto& [n2, item2] = arr[j];
      
      if (item1 == item2) {
        std::println("{} == {}", n1, n2);
      } else if (item1 < item2) {
        std::println("{} < {}", n1, n2);
      } else if (item1 > item2) {
        std::println("{} > {}", n1, n2);
      }
    }
  }
}

int main() {
  std::println("FLOAT");
  solution<float>();
  std::println("DOUBLE");
  solution<double>();
}
