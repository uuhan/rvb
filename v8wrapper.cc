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

/**
 * Associate embedder-specific data with the isolate. |slot| has to be
 * between 0 and GetNumberOfDataSlots() - 1.
 */
void
V8_Isolate_SetData(Isolate* isolate, uint32_t slot, void* data) {
    isolate->SetData(slot, data);
}

/**
 * Retrieve embedder-specific data from the isolate.
 * Returns NULL if SetData has never been called for the given |slot|.
 */
void*
V8_Isolate_GetData(Isolate* isolate, uint32_t slot) {
    return isolate->GetData(slot);
}

Isolate*
V8_FunctionCallbackInfo_GetIsolate(FunctionCallbackInfo<Value>& args) {
    return args.GetIsolate();
}

Local<Object>
V8_FunctionCallbackInfo_This(FunctionCallbackInfo<Value>& args, Local<Object>* out) {
    return args.This();
}
}
