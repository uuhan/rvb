#include <libplatform/libplatform.h>
#include <memory>


extern "C" {
    void new_platform(v8::Platform& platform) {
        std::unique_ptr<v8::Platform> p = v8::platform::NewDefaultPlatform();
        platform = *p.get();
    }
}

#include <v8.h>
