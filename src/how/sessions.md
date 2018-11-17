# Compiler sessions

The `rustc_driver` crate is what coordinates everything.

The `run_compiler` function is the meat of `main` in the compiler. It looks like this:

```rust
// Parse args and run the compiler. This is the primary entry point for rustc.
// See comments on CompilerCalls below for details about the callbacks argument.
// The FileLoader provides a way to load files from sources other than the file system.
pub fn run_compiler<'a>(args: &[String],
                        callbacks: Box<dyn CompilerCalls<'a> + sync::Send + 'a>,
                        file_loader: Option<Box<dyn FileLoader + Send + Sync + 'static>>,
                        emitter_dest: Option<Box<dyn Write + Send>>)
                        -> (CompileResult, Option<Session>)
{
    let matches = match handle_options(args) {
        Some(matches) => matches,
        None => return (Ok(()), None),
    };

    let (sopts, cfg) = config::build_session_options_and_crate_config(&matches);

    driver::spawn_thread_pool(sopts, |sopts| {
        run_compiler_with_pool(matches, sopts, cfg, callbacks, file_loader, emitter_dest)
    })
}
```

That was easy! Just kidding! Here's `run_compiler_with_pool`:

```rust
fn run_compiler_with_pool<'a>(
    matches: getopts::Matches,
    sopts: config::Options,
    cfg: ast::CrateConfig,
    mut callbacks: Box<dyn CompilerCalls<'a> + sync::Send + 'a>,
    file_loader: Option<Box<dyn FileLoader + Send + Sync + 'static>>,
    emitter_dest: Option<Box<dyn Write + Send>>
) -> (CompileResult, Option<Session>) {
    macro_rules! do_or_return {($expr: expr, $sess: expr) => {
        match $expr {
            Compilation::Stop => return (Ok(()), $sess),
            Compilation::Continue => {}
        }
    }}

    let descriptions = diagnostics_registry();

    do_or_return!(callbacks.early_callback(&matches,
                                           &sopts,
                                           &cfg,
                                           &descriptions,
                                           sopts.error_format),
                                           None);

    let (odir, ofile) = make_output(&matches);
    let (input, input_file_path, input_err) = match make_input(&matches.free) {
        Some((input, input_file_path, input_err)) => {
            let (input, input_file_path) = callbacks.some_input(input, input_file_path);
            (input, input_file_path, input_err)
        },
        None => match callbacks.no_input(&matches, &sopts, &cfg, &odir, &ofile, &descriptions) {
            Some((input, input_file_path)) => (input, input_file_path, None),
            None => return (Ok(()), None),
        },
    };

    let loader = file_loader.unwrap_or(box RealFileLoader);
    let source_map = Lrc::new(SourceMap::with_file_loader(loader, sopts.file_path_mapping()));
    let mut sess = session::build_session_with_source_map(
        sopts, input_file_path.clone(), descriptions, source_map, emitter_dest,
    );

    if let Some(err) = input_err {
        // Immediately stop compilation if there was an issue reading
        // the input (for example if the input stream is not UTF-8).
        sess.err(&err.to_string());
        return (Err(CompileIncomplete::Stopped), Some(sess));
    }

    let codegen_backend = get_codegen_backend(&sess);

    rustc_lint::register_builtins(&mut sess.lint_store.borrow_mut(), Some(&sess));

    let mut cfg = config::build_configuration(&sess, cfg);
    target_features::add_configuration(&mut cfg, &sess, &*codegen_backend);
    sess.parse_sess.config = cfg;

    let result = {
        let plugins = sess.opts.debugging_opts.extra_plugins.clone();

        let cstore = CStore::new(codegen_backend.metadata_loader());

        do_or_return!(callbacks.late_callback(&*codegen_backend,
                                              &matches,
                                              &sess,
                                              &cstore,
                                              &input,
                                              &odir,
                                              &ofile), Some(sess));

        let _sess_abort_error = OnDrop(|| sess.diagnostic().print_error_count());

        let control = callbacks.build_controller(&sess, &matches);

        driver::compile_input(codegen_backend,
                              &sess,
                              &cstore,
                              &input_file_path,
                              &input,
                              &odir,
                              &ofile,
                              Some(plugins),
                              &control)
    };

    (result, Some(sess))
}
```

The "session" is sort of like, well, global state for that particular compiler invocation.