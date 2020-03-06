extern crate cc;
extern crate gst_plugin_version_helper;

fn main() {
    gst_plugin_version_helper::get_info();

    cc::Build::new()
        .file("src/c/caption.c")
        .file("src/c/eia608.c")
        .file("src/c/eia608_charmap.c")
        .file("src/c/eia608_from_utf8.c")
        .file("src/c/utf8.c")
        .file("src/c/xds.c")
        .extra_warnings(false)
        .compile("libcaption-c.a");
}
