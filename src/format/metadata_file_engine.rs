trait MetadataFileEngine {
    fn extractor(&self) -> Box<dyn Extractor>;
    fn can_handle(file: DFSFile) -> bool;
}
