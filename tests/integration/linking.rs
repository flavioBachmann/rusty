use std::{env, fs};

use crate::get_test_file;
use inkwell::context::Context;
use rusty::{
    build_and_link, compile_module,
    diagnostics::{Diagnostic, Diagnostician},
    link, persist, CompileOptions, ErrorFormat, FilePath, FormatOption, LinkOptions, Target,
};

static TARGET: Option<&str> = Some("x86_64-unkown-linux-gnu");

#[test]
fn link_as_shared_object() {
    let file1 = FilePath {
        path: get_test_file("linking/file1.st"),
    };
    let file2 = FilePath {
        path: get_test_file("linking/file2.st"),
    };

    let mut out = env::temp_dir();
    out.push("shared1.so");
    let out1 = out.into_os_string().into_string().unwrap();
    let mut out = env::temp_dir();
    out.push("shared2.o");
    let out2 = out.into_os_string().into_string().unwrap();

    //Compile file 2 into obj
    build_and_link(
        vec![file2],
        vec![],
        None,
        &CompileOptions {
            output: out2.clone(),
            format: Some(FormatOption::Shared),
            optimization: rusty::OptimizationLevel::Default,
            error_format: ErrorFormat::Rich,
        },
        vec![TARGET.unwrap().into()],
        None,
        None,
    )
    .unwrap();

    //Compile file1 as shared object with file2 as param
    build_and_link(
        vec![file1, out2.as_str().into()],
        vec![],
        None,
        &CompileOptions {
            output: out1.clone(),
            format: Some(FormatOption::Shared),
            optimization: rusty::OptimizationLevel::Default,
            error_format: ErrorFormat::Rich,
        },
        vec![TARGET.unwrap().into()],
        None,
        Some(LinkOptions {
            libraries: vec![],
            library_pathes: vec![],
            format: FormatOption::Shared,
        }),
    )
    .unwrap();

    //Delete it
    fs::remove_file(&out1).unwrap();
    fs::remove_file(&out2).unwrap();
}

#[test]
fn link_as_pic_object() {
    let file1 = FilePath {
        path: get_test_file("linking/file1.st"),
    };
    let file2 = FilePath {
        path: get_test_file("linking/file2.st"),
    };

    let mut out = env::temp_dir();
    out.push("pic1.so");
    let out1 = out.into_os_string().into_string().unwrap();
    let mut out = env::temp_dir();
    out.push("pic2.o");
    let out2 = out.into_os_string().into_string().unwrap();

    //Compile file 2 into obj
    build_and_link(
        vec![file2],
        vec![],
        None,
        &CompileOptions {
            output: out2.clone(),
            format: Some(FormatOption::PIC),
            optimization: rusty::OptimizationLevel::Default,
            error_format: ErrorFormat::Rich,
        },
        vec![TARGET.unwrap().into()],
        None,
        None,
    )
    .unwrap();

    //Compile file1 as shared object with file2 as param
    build_and_link(
        vec![file1, out2.as_str().into()],
        vec![],
        None,
        &CompileOptions {
            output: out1.clone(),
            format: Some(FormatOption::PIC),
            optimization: rusty::OptimizationLevel::Default,
            error_format: ErrorFormat::Rich,
        },
        vec![TARGET.unwrap().into()],
        None,
        Some(LinkOptions {
            libraries: vec![],
            library_pathes: vec![],
            format: FormatOption::PIC,
        }),
    )
    .unwrap();
    //Delete it
    fs::remove_file(&out1).unwrap();
    fs::remove_file(&out2).unwrap();
}

#[test]
fn link_as_static_object() {
    let file1 = FilePath {
        path: get_test_file("linking/file1.st"),
    };
    let file2 = FilePath {
        path: get_test_file("linking/file2.st"),
    };

    let mut out = env::temp_dir();
    out.push("static1.o");
    let out1 = out.into_os_string().into_string().unwrap();
    let mut out = env::temp_dir();
    out.push("static2.o");
    let out2 = out.into_os_string().into_string().unwrap();

    //Compile file 2 into obj
    build_and_link(
        vec![file2],
        vec![],
        None,
        &CompileOptions {
            output: out2.clone(),
            format: Some(FormatOption::Static),
            optimization: rusty::OptimizationLevel::Default,
            error_format: ErrorFormat::Rich,
        },
        vec![TARGET.unwrap().into()],
        None,
        None,
    )
    .unwrap();

    //Compile file1 as shared object with file2 as param
    build_and_link(
        vec![file1, out2.as_str().into()],
        vec![],
        None,
        &CompileOptions {
            output: out1.clone(),
            format: Some(FormatOption::Static),
            optimization: rusty::OptimizationLevel::Default,
            error_format: ErrorFormat::Rich,
        },
        vec![TARGET.unwrap().into()],
        None,
        Some(LinkOptions {
            libraries: vec![],
            library_pathes: vec![],
            format: FormatOption::Static,
        }),
    )
    .unwrap();

    //Delete it
    fs::remove_file(&out1).unwrap();
    fs::remove_file(&out2).unwrap();
}

#[test]
fn link_as_relocatable_object() {
    let file1 = FilePath {
        path: get_test_file("linking/file1.st"),
    };
    let file2 = FilePath {
        path: get_test_file("linking/file2.st"),
    };

    let mut out = env::temp_dir();
    out.push("reloc1.o");
    let out1 = out.into_os_string().into_string().unwrap();
    let mut out = env::temp_dir();
    out.push("reloc2.o");
    let out2 = out.into_os_string().into_string().unwrap();

    //Compile file 2 into obj
    build_and_link(
        vec![file2],
        vec![],
        None,
        &CompileOptions {
            output: out2.clone(),
            format: Some(FormatOption::Static),
            optimization: rusty::OptimizationLevel::Default,
            error_format: ErrorFormat::Rich,
        },
        vec![TARGET.unwrap().into()],
        None,
        None,
    )
    .unwrap();

    //Compile file1 as shared object with file2 as param
    build_and_link(
        vec![file1, out2.as_str().into()],
        vec![],
        None,
        &CompileOptions {
            output: out1.clone(),
            format: Some(FormatOption::Relocatable),
            optimization: rusty::OptimizationLevel::Default,
            error_format: ErrorFormat::Rich,
        },
        vec![TARGET.unwrap().into()],
        None,
        Some(LinkOptions {
            libraries: vec![],
            library_pathes: vec![],
            format: FormatOption::Relocatable,
        }),
    )
    .unwrap();

    //Delete it
    fs::remove_file(&out1).unwrap();
    fs::remove_file(&out2).unwrap();
}

#[test]
fn link_missing_file() {
    let file1 = FilePath {
        path: get_test_file("linking/file1.st"),
    };
    let mut out = env::temp_dir();
    out.push("missing.o");
    let out = out.into_os_string().into_string().unwrap();
    let target: Target = TARGET.unwrap().into();
    //Compile file1 as shared object with file2 as param
    let context = Context::create();
    let diagnostician = Diagnostician::default();
    let (_, codegen) = compile_module(&context, vec![file1], vec![], None, diagnostician).unwrap();
    let object = persist(
        &codegen,
        &out,
        FormatOption::Static,
        &target.get_target_triple(),
        rusty::OptimizationLevel::Default,
    )
    .unwrap();
    let res = link(
        &out,
        FormatOption::Static,
        &[object],
        &[],
        &[],
        &target.get_target_triple(),
        None,
    );

    match res {
        Err(err) => {
            assert_eq!(Diagnostic::link_error(&format!("lld: error: undefined symbol: func2\n>>> referenced by main\n>>>               {}:(func1)\n>>> did you mean: func1\n>>> defined in: {}\n",out, out)), err);
        }
        _ => panic!("Expected link failure"),
    }

    //Delete it
    fs::remove_file(&out).unwrap();
}
