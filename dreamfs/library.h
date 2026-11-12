#ifndef DREAMFS_LIBRARY_H
#define DREAMFS_LIBRARY_H

#include <cstdint>
#include <string>
#include <string_view>
#include <vector>

namespace dreamfs {

struct FileInfo {
  std::string path;
  std::uint64_t size = 0;
};

std::string version();

std::vector<FileInfo> list(std::string_view root);

std::uint64_t total_size(std::vector<FileInfo> files);

}  // namespace dreamfs

#endif  // DREAMFS_LIBRARY_H
