pub trait FsFile {
    fn get_name() -> String;

    fn get_path() -> String;

    fn get_full_path() -> String;

    fn get_parent_path() -> String;

    fn get_parent_full_path() -> String;

    fn is_directory() -> bool;

    fn is_file() -> bool;

    fn get_length() -> u64;

    fn get_modification_time() -> Option<u64>;

    fn get_creation_time() -> Option<u64>;

    fn get_extra_attributes() -> Map<String, String>;
}
