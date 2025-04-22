// you will need to define a struct for your extension and implement the Extension trait, as well as use the register_extension! macro to register your extension

use zed_extension_api as zed;

struct MyExtension {
    // ... state
}

impl zed::Extension for MyExtension {
    // ...
}

zed::register_extension!(MyExtension);
