mod cli;

#[cfg(feature = "server")]
use cli::handle_run;
use cli::{
    build_command, handle_download, handle_generate, handle_info, handle_list, handle_preview,
};

fn main() {
    let matches = build_command().get_matches();

    match matches.subcommand() {
        Some(("list", _)) => {
            handle_list();
        }
        Some(("info", sub_matches)) => {
            handle_info(sub_matches);
        }
        Some(("preview", sub_matches)) => {
            handle_preview(sub_matches);
        }
        Some(("generate", sub_matches)) => {
            handle_generate(sub_matches);
        }
        Some(("download", sub_matches)) => {
            handle_download(sub_matches);
        }
        #[cfg(feature = "server")]
        Some(("run", sub_matches)) => {
            handle_run(sub_matches);
        }
        _ => {}
    }
}
