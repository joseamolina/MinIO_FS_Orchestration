use rayon::ThreadPoolBuilder;
use log::{info, debug};
use std::error::Error;

#[derive(Debug, Clone)]
struct CustomResource {
    path: String,
}

struct Engine;

struct File {
    full_path: String,
}


trait FsExtractor {

    fn check_exists(path: String) -> bool;

    fn files(path: String, recursive: bool);

    fn open_file(path: String);

    fn open_file_channel(path: String);

    fn close_file(path: String);

    fn seek(path: String, pos: u64);

    fn extract(path: &str) -> (Vec<String>, Vec<CustomResource>) {

        let pool = ThreadPoolBuilder::new().build().unwrap();
        info!("Created pool");

        let result = pool.install(|| {

            let all_files: Vec<(File, Option<Engine>)> = files_with_metadata_engines(path);

            let resources: Vec<CustomResource> = all_files
                .iter()
                .filter_map(|(file, maybe_engine)| {
                    if let Some(engine) = maybe_engine {
                        match engine.get_schema_as_custom_resource(file) {
                            Ok(custom_resource) => {
                                debug!("Extracted custom resource from {}", file.full_path);
                                Some(custom_resource)
                            }
                            Err(e) => {
                                debug!("Failed extracting resource from {}: {:?}", file.full_path, e);
                                None
                            }
                        }
                    } else {
                        None
                    }
                })
            .collect();
        });
    }

    fn files_with_metadata_engines(_path: &str) -> Vec<(File, Option<Engine>)> {
        vec![
            (File { full_path: "/data/file1".into() }, Some(Engine)),
            (File { full_path: "/data/file2".into() }, None),
        ]
    }

    fn extract_custom_path(file: &File) -> String {
        format!("custom:{}", file.full_path)
    }


}