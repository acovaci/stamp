use std::path::Path;

use crate::error::Res;

pub fn render_template(path: &Path, target: &Path) -> Res<()> {
    render_dir(&path.join("{{ cookiecutter.project_slug }}"), target)
}

pub fn render_dir(path: &Path, target: &Path) -> Res<()> {
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        let target = target.join(path.file_name().unwrap());

        println!("{}, {}", path.display(), target.display());

        if path.is_dir() {
            std::fs::create_dir(&target)?;
            render_dir(&path, &target)?;
        } else {
            render_file(&path, &target)?;
        }
    }
    Ok(())
}

pub fn render_file(path: &Path, target: &Path) -> Res<()> {
    let content = std::fs::read_to_string(path)?;
    let name = path.file_name().unwrap().to_str().unwrap();

    let mut env = minijinja::Environment::new();
    env.add_template(name, &content)?;
    let rendered = env.get_template(name)?.render(())?;

    std::fs::write(target, rendered)?;

    Ok(())
}
