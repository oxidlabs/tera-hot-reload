use std::time::Duration;
use std::path::Path;
use notify_debouncer_full::{
    Debouncer,
    DebounceEventResult,
    FileIdMap,
    new_debouncer,
};
use notify_debouncer_full::notify::{
    ReadDirectoryChangesWatcher, 
    EventKind, 
    RecursiveMode, 
    Watcher
};

pub fn watch<F>(reloader: F, delay: Duration, dirs: Vec<&'static str>) -> Debouncer<ReadDirectoryChangesWatcher, FileIdMap>
where
    F: Fn() + Send + 'static,
{
    let mut debouncer = new_debouncer(
        delay,
        None,
        move |result: DebounceEventResult| match result {
            Ok(events) => events.iter().for_each(|event| match event.kind {
                EventKind::Create(_) => {
                    println!("File created: {:?}", event);
                    reloader();
                }
                _ => {}
            }),
            Err(errors) => errors.iter().for_each(|error| println!("{error:?}")),
        },
    )
    .unwrap();

    for dir in &dirs {
        debouncer
            .watcher()
            .watch(Path::new(dir), RecursiveMode::Recursive)
            .unwrap();
    }

    debouncer
}