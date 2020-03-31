#![allow(unused)]

use crate::util;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::sync::{Arc, Mutex};
use zip::ZipArchive;

lazy_static! {
    static ref CPM: Mutex<ClassPathManager> = { Mutex::new(ClassPathManager::new()) };
}

pub fn init() {
    lazy_static::initialize(&CPM);
}

pub fn find_class(name: &str) -> Result<ClassPathResult, io::Error> {
    let cpm = CPM.lock().unwrap();
    cpm.search_class(name)
}

pub fn add_path(path: &str) {
    let mut cpm = CPM.lock().unwrap();
    let _ = cpm.add_class_path(path);
}

pub fn add_paths(path: &str) {
    let mut cpm = CPM.lock().unwrap();
    cpm.add_class_paths(path);
}

#[derive(Debug)]
pub struct ClassPathResult(pub String, pub Vec<u8>);

type ZipRef = Arc<Mutex<Box<ZipArchive<File>>>>;

enum ClassSource {
    Dir(String),
    Jar(ZipRef, String),
}

struct ClassPathManager {
    runtime_class_path: Vec<ClassSource>,
}

impl ClassPathManager {
    fn new() -> Self {
        Self {
            runtime_class_path: vec![],
        }
    }

    pub fn add_class_path(&mut self, path: &str) -> Result<(), io::Error> {
        let p = Path::new(path);
        if p.is_dir() {
            self.runtime_class_path
                .push(ClassSource::Dir(path.to_string()));
        } else {
            let f = File::open(p)?;
            let z = ZipArchive::new(f)?;
            let handle = Arc::new(Mutex::new(Box::new(z)));
            self.runtime_class_path
                .push(ClassSource::Jar(handle, path.to_string()));
        }

        Ok(())
    }

    pub fn add_class_paths(&mut self, path: &str) {
        path.split(util::PATH_SEP)
            .for_each(|p| match self.add_class_path(p) {
                Err(e) => error!("add class path error, path={}, e={:?}", p, e),
                _ => (),
            });
    }

    pub fn search_class(&self, name: &str) -> Result<ClassPathResult, io::Error> {
        let name = name.replace("/", util::FILE_SEP);
        let name = name.replace(".", util::FILE_SEP);

        // trace!("search_class: {}", name);

        for it in self.runtime_class_path.iter() {
            match it {
                ClassSource::Dir(path) => {
                    let mut p = String::from(path);
                    p.push_str(util::FILE_SEP);
                    p.push_str(&name);
                    p.push_str(".class");
                    match File::open(&p) {
                        Ok(mut f) => {
                            let mut v = Vec::with_capacity(f.metadata().unwrap().len() as usize);
                            let _ = f.read_to_end(&mut v);

                            return Ok(ClassPathResult(p, v));
                        }

                        _ => (),
                    }
                }

                ClassSource::Jar(handle, path) => {
                    let mut p = String::from(&name);
                    p.push_str(".class");

                    let mut handle = handle.lock().unwrap();
                    let zf = handle.by_name(&p);

                    match zf {
                        Ok(mut zf) => {
                            let mut v = Vec::with_capacity(zf.size() as usize);
                            let r = zf.read_to_end(&mut v);
                            assert!(r.is_ok());
                            return Ok(ClassPathResult(path.clone(), v));
                        }

                        _ => (),
                    }
                }
            }
        }

        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Search class failed: {}", name),
        ));
    }

    pub fn size(&self) -> usize {
        self.runtime_class_path.len()
    }
}
