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

const char* CopyString(std::string str) {
  size_t len = str.length();
  char* mem = (char*)malloc(len + 1);
  memcpy(mem, str.data(), len);
  mem[len] = 0;
  return mem;
}

const char* CopyString(String::Utf8Value& value) {
  if (value.length() == 0) {
    return nullptr;
  }
  return CopyString(*value);
}

void Fprint(FILE* out, const FunctionCallbackInfo<Value>& args) {
  bool first = true;
  for (int i = 0; i < args.Length(); i++) {
    Isolate* isolate = args.GetIsolate();
    HandleScope handle_scope(isolate);
    if (first) {
      first = false;
    } else {
      fprintf(out, " ");
    }
    String::Utf8Value str(isolate, args[i]);
    const char* cstr = CopyString(str);
    fprintf(out, "%s", cstr);
  }
  fprintf(out, "\n");
  fflush(out);
}

void Print(const FunctionCallbackInfo<Value>& args) {
  Fprint(stdout, args);
}

void Log(const FunctionCallbackInfo<Value>& args) {
  Fprint(stderr, args);
}

extern "C" {
static std::unique_ptr<Platform> defaultPlatform;

// Initialize V8
void v8_init() {
  if (defaultPlatform.get() == nullptr) {
    defaultPlatform = platform::NewDefaultPlatform();
    V8::InitializePlatform(defaultPlatform.get());
    V8::Initialize();
  }
}

m_engine* v8_new() {
  Isolate::CreateParams params;
  params.array_buffer_allocator = defaultAllocator;
  Isolate* isolate = Isolate::New(params);

  Locker locker(isolate);
  Isolate::Scope isolate_scope(isolate);
  HandleScope handle_scope(isolate);

  isolate->SetCaptureStackTraceForUncaughtExceptions(true);

  Local<ObjectTemplate> global = ObjectTemplate::New(isolate);
  Local<ObjectTemplate> v8engine = ObjectTemplate::New(isolate);

  global->Set(isolate, "V8Engine", v8engine);

  v8engine->Set(isolate, "print", FunctionTemplate::New(isolate, Print));
  v8engine->Set(isolate, "log", FunctionTemplate::New(isolate, Log));
  // v8engine->Set(isolate, "cb", FunctionTemplate::New(isolate, cb));

  m_engine* engine = new m_engine;
  engine->ptr.Reset(isolate, Context::New(isolate, NULL, global));
  engine->isolate = isolate;
  isolate->SetData(0, engine);
  return engine;
}

int v8_run(m_engine* ptr, const char* origin, const char* source) {
  m_engine* engine = static_cast<m_engine*>(ptr);
  Isolate* isolate = engine->isolate;
  Locker locker(isolate);
  Isolate::Scope isolate_scope(isolate);
  HandleScope handle_scope(isolate);
  TryCatch try_catch(isolate);

  Local<Context> lContext = engine->ptr.Get(isolate);
  Context::Scope context_scope(lContext);

  Local<String> lSource =
      String::NewFromUtf8(isolate, source, NewStringType::kNormal)
          .ToLocalChecked();
  Local<String> lOrigin =
      String::NewFromUtf8(isolate, source, NewStringType::kNormal)
          .ToLocalChecked();

  ScriptOrigin script_origin(lOrigin);
  MaybeLocal<Script> script =
      Script::Compile(lContext, lSource, &script_origin);
  if (script.IsEmpty()) {
    return 1;
  }

  MaybeLocal<v8::Value> result = script.ToLocalChecked()->Run(lContext);
  if (result.IsEmpty()) {
    return 2;
  }

  return 0;
}

void v8_delete(m_engine* ptr) {
  ptr->isolate->Dispose();
}

const char* v8_version() {
  return V8::GetVersion();
}
}
