#include "library.h"

#include <filesystem>
#include <system_error>

namespace dreamfs {
namespace {

std::uint64_t entry_size(const std::filesystem::directory_entry& entry) {
  std::error_code error;
  const auto size = entry.file_size(error);
  return error ? 0 : static_cast<std::uint64_t>(size);
}

}  // namespace

std::string version() { return "0.1.0"; }

std::vector<FileInfo> list(std::string_view root) {
  std::vector<FileInfo> files;
  std::error_code error;
  for (const auto& entry :
       std::filesystem::recursive_directory_iterator(root, error)) {
    if (error) {
      break;
    }
    if (!entry.is_regular_file(error)) {
      continue;
    }
    files.push_back(FileInfo{entry.path().string(), entry_size(entry)});
  }
  return files;
}

std::uint64_t total_size(std::vector<FileInfo> files) {
  std::uint64_t total = 0;
  for (const auto& file : files) {
    total += file.size;
  }
  return total;
}

}  // namespace dreamfs
