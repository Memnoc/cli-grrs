pub fn find_matches(
    content: &str,
    pattern: &str,
    mut writer: impl std::io::Write,
) -> Result<(), std::io::Error> {
    let pb = indicatif::ProgressBar::new(content.lines().count() as u64);
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line)?;
            pb.println(format!("[+] finished #{}", line));
            pb.inc(1);
        }
    }
    pb.finish_with_message("done");
    Ok(())
}
