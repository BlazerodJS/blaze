use cargo_gn;
use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::process::exit;
use std::process::Command;
use which::which;

// mod src::typescript;

fn main() {
  // git submodule update --init --recursive
  let libcxx_src = PathBuf::from("buildtools/third_party/libc++/trunk/src");
  if !libcxx_src.is_dir() {
    eprintln!("missing source code. Run 'git submodule update --init --recursive'");
    exit(1);
  }

  env::set_var("DEPOT_TOOLS_WIN_TOOLCHAIN", "0");
  env::set_var("PYTHONDONTWRITEBYTECODE", "1");

  if env::var_os("RUSTDOCFLAGS").is_some() {
    exit(1);
  }

  download_gn_ninja_binaries_if_necessary();

  let mut gn_args = if cargo_gn::is_debug() && !cfg!(target_os = "windows") {
    vec!["is_debug=true".to_string()]
  } else {
    vec!["is_debug=false".to_string()]
  };

  let clang_base_path = download_clang();
  gn_args.push(format!("clang_base_path={:?}", clang_base_path));

  if let Some(p) = env::var_os("SCCACHE") {
    cc_wrapper(&mut gn_args, &Path::new(&p));
  } else if let Ok(p) = which("sccache") {
    cc_wrapper(&mut gn_args, &p);
  } else {
    println!("cargo:warning=Not using sccache");
  }

  let gn_root = env::var("CARGO_MANIFEST_DIR").unwrap();

  let gn_out = cargo_gn::maybe_gen(&gn_root, gn_args);
  assert!(gn_out.exists());
  assert!(gn_out.join("args.gn").exists());
  cargo_gn::build("blazerod");

  println!("cargo:rustc-link-lib=static=blazerod");

  if cfg!(target_os = "windows") {
    println!("cargo:rustc-link-lib=dylib=winmm");
    println!("cargo:rustc-link-lib=dylib=dbghelp");
  }

  // println!("cargo:rustc-env=TS_VERSION={}", typescript::ts_version());

  // let c = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
  // let o = PathBuf::from(env::var_os("OUT_DIR").unwrap());

  // let root_names = vec![c.join("js/main.ts")];
  // let bundle = o.join("CLI_SNAPSHOT.js");
  // let state = typescript::compile_bundle(&bundle, root_names).unwrap();
  // assert!(bundle.exists());
  // typescript::mksnapshot_bundle(&bundle, state).unwrap();

  // let root_names = vec![c.join("js/compiler.ts")];
  // let bundle = o.join("COMPILER_SNAPSHOT.js");
  // let state = typescript::compile_bundle(&bundle, root_names).unwrap();
  // assert!(bundle.exists());
  // typescript::mksnapshot_bundle_ts(&bundle, state).unwrap();
}

fn platform() -> &'static str {
  #[cfg(target_os = "linux")]
  {
    "linux64"
  }
  #[cfg(target_os = "macos")]
  {
    "mac"
  }
  #[cfg(target_os = "windows")]
  {
    "win"
  }
}

fn download_gn_ninja_binaries_if_necessary() {
  if (which("ninja").is_ok() || env::var_os("NINJA").is_some()) && env::var_os("GN").is_some() {
    return;
  }

  let root = env::current_dir().unwrap();
  let out_dir = root.join(env::var_os("OUT_DIR").unwrap());
  let d = out_dir.join("gn_ninja_binaries").join(platform());
  let gn = d.join("gn");
  let ninja = d.join("ninja");
  #[cfg(windows)]
  let gn = gn.with_extension("exe");
  #[cfg(windows)]
  let ninja = ninja.with_extension("exe");

  if !gn.exists() || !ninja.exists() {
    let status = Command::new("python")
      .arg("./tools/gn_ninja_binaries.py")
      .arg("--dir")
      .arg(&out_dir)
      .status()
      .expect("gn_ninja_binaries.py download failed");
    assert!(status.success());
  }

  assert!(gn.exists());
  assert!(ninja.exists());

  env::set_var("GN", gn);
  env::set_var("NINJA", ninja);
}

fn download_clang() -> PathBuf {
  // if let Ok(clang_path) = which("clang") {
  //   let bin_path = clang_path.parent().unwrap();
  //   return bin_path.parent().unwrap().to_path_buf();
  // }

  let root = env::current_dir().unwrap();
  let out_dir = env::var_os("OUT_DIR").unwrap();
  let clang_base_path = root.join(out_dir).join("clang");
  println!("clang_base_path {}", clang_base_path.display());
  let status = Command::new("python")
    .arg("./tools/clang/scripts/update.py")
    .arg("--output-dir")
    .arg(&clang_base_path)
    .status()
    .expect("clang download failed");
  assert!(status.success());
  assert!(clang_base_path.exists());
  clang_base_path
}

fn cc_wrapper(gn_args: &mut Vec<String>, sccache_path: &Path) {
  gn_args.push(format!("cc_wrapper={:?}", sccache_path));
  // Disable treat_warnings_as_errors until this sccache bug is fixed:
  // https://github.com/mozilla/sccache/issues/264
  if cfg!(target_os = "windows") {
    gn_args.push("treat_warnings_as_errors=false".to_string());
  }
}
