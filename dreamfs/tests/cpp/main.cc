#include <iostream>
#include <string>

#include "library.h"

int main() {
  if (dreamfs::version() != "0.1.0") {
    std::cerr << "unexpected version: " << dreamfs::version() << '\n';
    return 1;
  }
  std::cout << "dreamfs " << dreamfs::version() << '\n';

  const std::vector<dreamfs::FileInfo> files = dreamfs::list(".");
  std::cout << files.size() << " files, " << dreamfs::total_size(files)
            << " bytes\n";
  return 0;
}
