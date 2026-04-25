use dialoguer::Select;
use super::super::output::Output;
use super::super::input;

pub fn handle(shell: &Option<String>, output: &Output) {
    let shell = match shell {
        Some(s) => s.clone(),
        None => {
            let shells = ["bash", "zsh", "fish", "powershell", "elvish"];
            let sel = match Select::new()
                .with_prompt("选择目标 Shell")
                .items(&shells)
                .interact()
            {
                Ok(s) => s,
                Err(_) => { output.error("已取消"); return; }
            };
            shells[sel].to_string()
        }
    };

    output.line(format!("Shell 补全脚本生成功能尚未实现 (目标: {})", shell));
    output.hint("可使用 clap 的补全生成功能: https://docs.rs/clap/latest/clap/");
}
