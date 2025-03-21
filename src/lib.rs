use notify_debouncer_full::notify::{ EventKind, RecommendedWatcher, RecursiveMode };
use notify_debouncer_full::{ new_debouncer, DebounceEventResult, Debouncer, RecommendedCache };
use std::path::Path;
use std::time::Duration;

pub use tera_template_macro::TeraTemplate;
pub use tower_livereload::LiveReloadLayer;

/// Watches the specified directories for changes and triggers the provided reloader function when a file is created, modified or deleted.
///
/// # Arguments
///
/// * `reloader`: A closure that will be executed when a change is detected. This closure should return void (`()`) and be Send-compatible.
/// * `delay`: The minimum duration between checks for changes on the watched directories.
/// * `dirs`: An array of directory paths to watch for changes. Each path may be absolute or relative, but must exist on the system's file system.
///
/// # Returns
///
/// A Debouncer that will trigger the reloader function at regular intervals, and watch the specified directories for changes.
///
/// # Examples
///
/// ```
/// let livereload = LiveReloadLayer::new();
/// let reload = livereload.reloader();
///
/// let _debouncer = watch(move || reloader.reload(), Duration::from_millis(10), vec!["./src"]);
/// ```
pub fn watch<F, D, P>(
    reloader: F,
    delay: Duration,
    dirs: D
)
    -> Debouncer<RecommendedWatcher, RecommendedCache>
    where F: Fn() + Send + 'static, D: IntoIterator<Item = P>, P: AsRef<Path>
{
    let mut debouncer = new_debouncer(delay, None, move |result: DebounceEventResult| {
        match result {
            Ok(events) =>
                events.iter().for_each(|event| {
                    match event.kind {
                        EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_) => {
                            reloader();
                        }
                        _ => {}
                    }
                }),
            Err(errors) => errors.iter().for_each(|error| println!("{error:?}")),
        }
    }).unwrap();

    for dir in dirs.into_iter() {
        debouncer.watch(dir.as_ref(), RecursiveMode::Recursive).unwrap();
    }

    debouncer
}
