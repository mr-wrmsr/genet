#ifndef PLUGKIT_FILE_EXPORTER_THREAD_H
#define PLUGKIT_FILE_EXPORTER_THREAD_H

#include "token.h"
#include <functional>
#include <memory>
#include <string>

namespace plugkit {

class Frame;
struct FileExporter;

class Logger;
using LoggerPtr = std::shared_ptr<Logger>;

class FrameStore;
using FrameStorePtr = std::shared_ptr<FrameStore>;

class FileExporterThread final {
public:
  using Callback = std::function<void(double)>;

public:
  FileExporterThread(const FrameStorePtr &store);
  ~FileExporterThread();
  void setLogger(const LoggerPtr &logger);
  void setCallback(const Callback &callback);
  void addExporter(const FileExporter &exporter);
  void registerLinkLayer(Token token, int link);
  bool start(const std::string &file, const std::string &filter);

private:
  class Private;
  std::unique_ptr<Private> d;
};

} // namespace plugkit

#endif
