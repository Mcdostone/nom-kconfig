use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf}, borrow::BorrowMut,
};

use nom::{
    IResult, combinator::map,
};
use regex::Regex;

use crate::{
    entry::{
        /*source::{Source},*/ parse_entries,
    },
    kconfig::{Kconfig}, KconfigInput,
    // Entry, KconfigInput,
};

#[derive(Debug, Clone)]
pub struct Parser {
    kernel_dir: PathBuf,
    files: HashMap<String, String>,
}



impl Parser {
    pub fn new(kernel_dir: PathBuf) -> Self {
        Self {
            kernel_dir,
            files: HashMap::new(),
        }
    }

    fn is_architecture_related(kconfig: &Path, kernel_dir: &Path) -> Option<String> {
        match kconfig.starts_with(kernel_dir.join("arch")) {
            true => {
                let ok = kconfig
                    .strip_prefix(kernel_dir)
                    .unwrap()
                    .components()
                    .nth(1)
                    .unwrap();
                Some(ok.to_owned().as_os_str().to_str().unwrap().to_string())
            }
            false => None,
        }
    }


    fn absolute_path(kernel_dir: &Path, p: &str) -> PathBuf {
        kernel_dir.join(p)
    }

    fn process_input(input: &str) -> String {
        let re = Regex::new("\\\\\n").unwrap();
        let input = re.replace_all(input, "");
        let re = Regex::new("\t").unwrap();
        let input = re.replace_all(&input, "    ");
        input.to_string()
    }

    pub fn parse_file(&mut self, filename: &Path) -> IResult<KconfigInput, Kconfig> {
        let kernel_dir = self.kernel_dir.clone();
        let file = filename.display().to_string();
        let input = fs::read_to_string(filename).unwrap();
        let input = Self::process_input(&input);
        self.files.insert(file.clone(), input);
        let input = self.files.borrow_mut().get(&file).unwrap();
        let input = KconfigInput::new_extra(input, kernel_dir);
        map(parse_entries, |entries| {
            let kconfig = Kconfig {
                file: filename.display().to_string(),
                entries,
                architecture: Self::is_architecture_related(filename, &self.kernel_dir.clone()),
            };
            return kconfig
        })(input)

        //let (_, mut kconfig) = self.parse_f(filename).unwrap();
        //let sources = Self::get_sources(&mut kconfig);
        //for source in sources {
        //    let source_filename = Self::absolute_path(&kernel_dir, &source.file);
        //    match self.parse_f(&source_filename) {
        //        Ok((_, k)) => {
        //            source.kconfig = k
        //        },
        //        Err(_) => {
        //            //eprintln!("Cannot parse {}: {}", source.file.clone(), e);
        //            panic!()
        //        }, 
        //    }
        //}
        //Ok(("", kconfig))
        //}

        /*match parse_entries(input) {
            Ok((remaining, entries)) => {
                let kconfig = Kconfig {
                    file: file.clone(),
                    entries,
                    architecture: Self::is_architecture_related(filename, kernel_dir),
                };

                let sources = Self::get_sources(kconfig.clone());
                for mut source in sources {
                    let source_filename = self.absolute_path(&source.file);
                    println!("{:?}", &source_filename);
                    let input_source: String = fs::read_to_string(source_filename).unwrap();
                    self.files.insert(source.file.clone(), Self::process_input(&input_source));
                    let input = self.files.get(&source.file).unwrap();
                    //println!("{}", input);
                    match parse_entries(input) {
                        Ok((remaining, entries)) => {
                            source.kconfig = Kconfig {
                                file: source.file,
                                entries: entries,
                                architecture: None,
                            };
                        }
                        Err(e) => {
                            eprintln!("Cannot parse {}: {}", source.file.clone(), e);
                            panic!()
                        }
                        Err(e) => {
                            eprintln!("Cannot parse {}: {}", source.file.clone(), e);
                            panic!()
                        }
                    }
                }
                //Ok((remaining, kconfig))
                Ok(("", kconfig))
            }
            Err(e) => {
                //eprintln!("Cannot parse {}: {}", file, e);
                panic!()
            }
        }*/
    }

/* 
    pub fn parse_f(&mut self, filename: &Path) -> IResult<&str, Kconfig,> {
        let file = filename.display().to_string();
        let input = fs::read_to_string(filename).unwrap();
        let input = Self::process_input(&input);
        self.files.insert(file.clone(), input);
        let input = self.files.borrow_mut().get(&file).unwrap();
        map(parse_entries, |entries| {
            let kconfig = Kconfig {
                file: filename.display().to_string(),
                entries,
                architecture: Self::is_architecture_related(filename, &self.kernel_dir.clone()),
            };
            return kconfig
        })(input)
    }

    pub fn get_sources(kconfig: &mut Kconfig) -> Vec<&mut Source> {
        let mut sources = vec![];
        for ok in kconfig.entries.iter_mut() {
            if let Entry::Source(a) = ok {
                sources.push(a)
            }
        }
        sources
    }

    pub fn parse_source<'a>(&'a self, input: &'a str, source: &'a mut Source) -> IResult<&str, &Source> {
        match parse_entries(input) {
            Ok((remaining, entries)) => {
                let f =  PathBuf::from(source.file.clone());
                source.kconfig = Kconfig {
                    file: source.file.to_string(),
                    entries,
                    architecture: Self::is_architecture_related(f.as_path(), &self.kernel_dir.clone()),
                };
                Ok((remaining, source))
            }
            Err(e) => {
                eprintln!("Cannot parse {}: {}", source.file.clone(), e);
                panic!()
            }
        }
    }*/
}
