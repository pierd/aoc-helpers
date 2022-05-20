use std::{env, fs::File, io::Write};

fn main() -> Result<(), anyhow::Error> {
    let mut args = env::args();
    args.next();
    let day_num = args.next().ok_or(anyhow::anyhow!("Provide day number"))?.parse::<usize>()?;
    let day_str = format!("{:02}", day_num);
    let file_contents = include_str!("template.rs.tmpl").replace("_N_", &day_str);
    let output_path = format!("src/bin/day{}.rs", day_str);
    write!(File::options().write(true).create_new(true).open(output_path)?, "{}", file_contents)?;
    Ok(())
}