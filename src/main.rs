
use colored::*;
use exec;
use std::env;
use std::io::Error as ioError;
use std::path::PathBuf;
use structopt::StructOpt;


#[derive(Debug)]
#[derive(StructOpt)]
#[structopt(setting=structopt::clap::AppSettings::AllowLeadingHyphen)]
#[structopt(setting=structopt::clap::AppSettings::TrailingVarArg)]
struct ArgumentParser {

  #[structopt(help="cmd workdir", parse(try_from_str=parse_cwd))]
  cwd: PathBuf,

  #[structopt(help="cmd to exec")]
  cmd: String,

  #[structopt(help="cmd args")]
  arg: Vec<String>,
}


fn parse_cwd(s: &str) -> Result<PathBuf, ioError> {
  let dirpath = PathBuf::from(s);
  if dirpath.is_dir() {
    Ok(dirpath)
  }
  else {
    Err(ioError::from_raw_os_error(2 as i32))
  }
}


fn log_exec_error(error: exec::Error, workdir: PathBuf, command: String) {
  let msg = "error:".red();
  let cmd = format!("<{}>", &command.trim());
  let dir = format!("{:?}", &workdir).blue();
  let err = format!("{}", &error).red();
  println!("{msg} executing {cmd} at {dir} {err}", msg=msg, cmd=cmd.yellow(), dir=dir, err=err)
}


fn main() {
  let opts = ArgumentParser::from_args();

  env::set_current_dir(&opts.cwd).expect("Setting the current workdir");
  let workdir = env::current_dir().expect("Getting fullpath of the current workdir");
  env::set_var("PWD", &workdir);

  let error = exec::Command::new(&opts.cmd).args(&opts.arg).exec();
  log_exec_error(error, workdir, format!("{} {}", &opts.cmd, &opts.arg.join(" ")));
}
