// v8 c wrapper
#include <libplatform/libplatform.h>
#include <v8.h>

#include <memory>

using namespace v8;

class ArrayBufferAllocator : public ArrayBuffer::Allocator
{
public:
    static ArrayBufferAllocator the_singleton;
    virtual void *Allocate(size_t length);
    virtual void *AllocateUninitialized(size_t length);
    virtual void Free(void *data, size_t);
};

ArrayBufferAllocator ArrayBufferAllocator::the_singleton;

void *
ArrayBufferAllocator::Allocate(size_t length)
{
    void *data = AllocateUninitialized(length);
    return data == NULL ? data : memset(data, 0, length);
}

void *
ArrayBufferAllocator::AllocateUninitialized(size_t length)
{
    return malloc(length);
}

void
ArrayBufferAllocator::Free(void *data, size_t)
{
    free(data);
}

extern "C" {
static v8::Platform *default_platform;
static ArrayBufferAllocator array_buffer_allocator;

Isolate *isolate;
Local<Context> context;

bool
V8_Initialize_Platform()
{
    if (default_platform == nullptr) {
        std::unique_ptr<v8::Platform> platform =
            v8::platform::NewDefaultPlatform();
        v8::V8::InitializePlatform(platform.get());
    }
    return true;
}

bool
V8_Initialize()
{
    return V8::Initialize();
}

bool
V8_Dispose()
{
    return V8::Dispose();
}

bool
V8_Free_Platform() {
    delete default_platform;
    default_platform = nullptr;
    return true;
}

bool
V8_Isolate_New()
{
    Isolate::CreateParams create_params;
    create_params.array_buffer_allocator =
        ArrayBuffer::Allocator::NewDefaultAllocator();
    isolate = Isolate::New(create_params);
}

void
V8_Isolate_Dispose()
{
    isolate->Dispose();
    isolate = nullptr;
}

void
V8_Isolate_Enter()
{
    isolate->Enter();
}

void
V8_Isolate_Exit()
{
    isolate->Exit();
}

void
V8_Context_New()
{
    context = Context::New(isolate);
}

void
V8_Context_Enter()
{
    context->Enter();
}

void
V8_Context_Exit()
{
    context->Enter();
}
}
