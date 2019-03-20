// v8 c wrapper
#include "v8wrapper.h"
#include <memory>

using namespace v8;

extern "C" {

std::unique_ptr<Platform>
V8_Initialize_Platform()
{
    std::unique_ptr<v8::Platform> platform = v8::platform::NewDefaultPlatform();
    v8::V8::InitializePlatform(platform.get());
    V8::Initialize();
    return platform;
}

void
V8_Get_Platform(std::unique_ptr<Platform> platform, Platform** out) {
    *out = platform.get();
}

Local<Value>
V8_To_Local_Checked(v8::MaybeLocal<v8::Value> value) {
    return value.ToLocalChecked();
}
}
