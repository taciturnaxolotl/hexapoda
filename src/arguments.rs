use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Arguments {
	/// specify a file to use for configuration
	#[arg(short, long, value_name = "file")]
	pub config: Option<PathBuf>,
	
	/// the input files to edit
	#[arg(value_name = "files")]
	pub files: Vec<PathBuf>,
	
	/// print the path to the config file
	#[arg(short, long)]
	pub show_config_path: bool,
}
