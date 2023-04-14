use std::{
    ffi::OsStr,
    fs::{self, File},
    io::Write,
};

fn main() {
    match fs::read_dir("./bindings") {
        Ok(dir) => {
            let exports: Vec<_> = dir
                .filter_map(Result::ok)
                .filter_map(|p| {
                    p.path()
                        .file_stem()
                        .map(OsStr::to_str)
                        .flatten()
                        .map(str::to_owned)
                })
                .filter(|f| f != "index")
                .map(|f| format!("export * from \"./{}\"", f))
                .collect();
            let mut file = File::create("./bindings/index.ts").unwrap();
            file.write_all(exports.join("\n").as_bytes()).unwrap();

            match fs_extra::dir::move_dir(
                "./bindings",
                "../../../orchid_admin_ui/src/types",
                &fs_extra::dir::CopyOptions::new().overwrite(true),
            ) {
                Ok(res) => {
                    println!("{:#?}", res)
                }
                Err(err) => {
                    println!("{:#?}", err)
                }
            }
        }
        Err(_) => {}
    }
}
