#include "v8/include/v8.h"

#include "v8/include/libplatform/libplatform.h"

#include <stdio.h>
#include <stdlib.h>
#include <cstdlib>
#include <cstring>
#include <map>
#include <sstream>
#include <string>

using namespace v8;

auto defaultAllocator = ArrayBuffer::Allocator::NewDefaultAllocator();
// auto defaultPlatform = platform::NewDefaultPlatform();

typedef struct {
  Persistent<Context> ptr;
  Isolate* isolate;

  Persistent<Function> cb;

  std::map<std::string, Eternal<Module>> modules;
  std::map<int, std::map<std::string, Eternal<Module>>> resolved;
} m_engine;

extern "C" {
static std::unique_ptr<Platform> defaultPlatform;

// Initialize V8
void init_v8() {
  if (defaultPlatform.get() == nullptr) {
    defaultPlatform = platform::NewDefaultPlatform();
    V8::InitializePlatform(defaultPlatform.get());
    V8::Initialize();
  }
}

const char* v8_version() {
  return V8::GetVersion();
}
}
