use std::{
    ffi::OsStr,
    fs::{self, File},
    io::Write,
    path::PathBuf,
    process::Command,
};

fn run_dir() -> PathBuf {
    std::env::current_dir().unwrap()
}

fn current_dir() -> PathBuf {
    run_dir().join("app").join("cli").join("bindings")
}

fn types_dir() -> PathBuf {
    run_dir()
        .join("..")
        .join("orchid_admin_ui")
        .join("src")
        .join("types")
}

pub fn init() {
    let dir = run_dir();
    if !current_dir().is_dir() {
        fs::create_dir_all(current_dir()).unwrap();
    }
    let exports = vec![dir.join("app").join("admin"), dir.join("service")];

    run_test("service");
    run_test("admin");
    for export in exports {
        merge_file(export);
    }
    move_dir();
}

fn run_test(name: &str) {
    let output = Command::new("cargo")
        .arg("test")
        .arg("-p")
        .arg(name)
        .output()
        .expect("failed to execute process");
    std::io::stdout().write_all(&output.stdout).unwrap();
}
/// 复制ts文件到当前目录下
fn merge_file(dir: PathBuf) {
    let dir_path = dir.join("bindings");
    if dir_path.is_dir() {
        for entry in dir_path.read_dir().unwrap() {
            let entry = entry.unwrap();
            fs_extra::file::copy(
                entry.path(),
                current_dir().join(entry.file_name()),
                &fs_extra::file::CopyOptions::new().overwrite(true),
            )
            .unwrap();
        }
        let exports: Vec<_> = fs::read_dir(current_dir())
            .unwrap()
            .filter_map(Result::ok)
            .filter_map(|p| {
                p.path()
                    .file_stem()
                    .and_then(OsStr::to_str)
                    .map(str::to_owned)
            })
            .filter(|f| f != "index")
            .map(|f| format!("export * from \"./{}\"", f))
            .collect();

        let mut file = File::create(current_dir().join("index.ts")).unwrap();
        file.write_all(exports.join("\n").as_bytes()).unwrap();
    }
}

/// 移动bingings目录到前端源码类型目录
fn move_dir() {
    // println!("{:#?}", types_dir());
    fs_extra::dir::move_dir(
        current_dir(),
        types_dir(),
        &fs_extra::dir::CopyOptions::new().overwrite(true),
    )
    .unwrap();
}
