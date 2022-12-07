use std::{path::{PathBuf}, io::{BufReader, BufRead}, fs::File, collections::{HashSet, HashMap}, cell::RefCell};


/// cd x
/// cd ..
/// cd /
/// ls 
///   123 abc
///   dir xyz
enum Command {
    Cd {to: String, },
    Ls,
}



#[derive(Debug, Default)]
struct Directory {
    cwd: PathBuf, // Path to the directory
    files: HashMap<String, usize>,
    dirs: HashSet<PathBuf>,
    size: RefCell<Option<usize>>, // sum of files and sub directory files
}

impl Directory {
    fn new(cwd: PathBuf) -> Self {
        Self {
            cwd,
            files: HashMap::new(),
            dirs: HashSet::new(),
            size: RefCell::new(None),
        }
    }
}


pub fn day7(path: &PathBuf) -> std::io::Result<()> {

    let lines = BufReader::new(File::open(path)?).lines();

    let mut hm_directories = HashMap::new();
    let mut working_directory = PathBuf::new();

    for line in lines {
        match line {
            Ok(line) => {
                if line.starts_with("$") {
                    // this line is a command
                    run_command(line.into(), &mut working_directory);
                    if !hm_directories.contains_key(&working_directory) {
                        hm_directories.insert(working_directory.clone(), Directory::new(working_directory.clone()));
                    }
                } else {
                    let v = line.split(" ").collect::<Vec<&str>>();
                    if v[0].starts_with("dir") {
                        // dir xyz
                        let mut sub_dir = working_directory.clone();
                        sub_dir.push(v[1]);
                        hm_directories.get_mut(&working_directory).unwrap().dirs.insert(sub_dir);
                    } else {
                        // 123 abc
                        hm_directories.get_mut(&working_directory).unwrap().files.insert(v[1].into(), v[0].parse().unwrap());
                    }
                }
            },
            Err(_) => todo!(),
        }
    }


    calc_size(&PathBuf::from("/"), &hm_directories);


    let mut sum = 0;
    for (_, dir) in &hm_directories {
        let cur_size = dir.size.borrow().unwrap();
        if cur_size <= 100000 {
            sum += cur_size;
        }
    }

    println!("sum: {}", sum);

    let disk_space = 70000000;
    let need_unused = 30000000;
    let unused_size = disk_space - hm_directories.get(&PathBuf::from("/")).unwrap().size.borrow().unwrap();
    let need_free = need_unused - unused_size;

    let mut min = disk_space;

    for (_, dir) in &hm_directories {
        let cur_size = dir.size.borrow().unwrap();
        if cur_size >= need_free {
            if cur_size < min {
                min = cur_size;
            }
        }
    }
    
    println!("min: {}", min);

    Ok(())
}


fn calc_size(path: &PathBuf, hm_directories: &HashMap<PathBuf, Directory>) -> usize {
    match hm_directories.get(path) {
        Some(dir) => {
            match *dir.size.borrow() {
                Some(size) => return size,
                None => (),
            }

            let mut sum_size = 0;
            for file in dir.files.iter() {
                sum_size += file.1;
            }
            for dir in dir.dirs.iter() {
                sum_size += calc_size(dir, &hm_directories);
            }

            *dir.size.borrow_mut() = Some(sum_size);

            // println!("{}: {}", dir.cwd.to_str().unwrap(), sum_size);

            sum_size
        },
        None => 0,
    }
}

fn run_command(cmd: Command, working_directory: &mut PathBuf) {
    match cmd {
        Command::Cd { to } => {
            match to.as_ref() {
                ".." => { working_directory.pop(); },
                "/" => *working_directory = PathBuf::from("/"),
                x => working_directory.push(x),
            }
        },
        Command::Ls => (),
    }
}

impl<T: AsRef<str>> From<T> for Command {
    fn from(s: T) -> Self {
        let v = s.as_ref().split(" ").collect::<Vec<&str>>();
        match v[1] {
            "cd" => Command::Cd { to: v[2].to_string() },
            "ls" => Command::Ls,
            _ => panic!("Invalid command"),
        }
    }
}