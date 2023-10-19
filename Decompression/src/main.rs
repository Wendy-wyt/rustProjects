use std::fs;
use std::io;

fn main(){
    // Allow the program exit with an exit code
    std::process::exit(real_main());
}

fn real_main() ->i32{
    // get input and output paths of file
    println!("Please enter the input file: ");
    let mut input=String::new();
    io::stdin().read_line(&mut input).unwrap();

    // access the file and decompress
    let fname = std::path::Path::new(&*input.trim());
    let file=fs::File::open(&fname).unwrap();
    let mut archive=zip::ZipArchive::new(file).unwrap();

    // iterate over the files in archive to maintain the structure of zip file
    for i in 0..archive.len(){
        let mut file = archive.by_index(i).unwrap();
        // get the path name of the current file
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(), // clone to new path
            None => continue,
        };
        // since the file can be a directory
        // we need to create new dir for the directory
        if file.is_dir() {
            println!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        }else{
            println!("File {} extracted to \"{}\" ({} bytes)", i, outpath.display(),file.size());
            // if the file requires a parent but the parent doesn't exist, create dir
            if let Some(p)=outpath.parent(){
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }

            // now copy the extracted file to the output location
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }

    return 0;
}