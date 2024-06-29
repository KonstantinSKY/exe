


match fs::create_dir(&app_path.join(path) {
    Ok(()) => {
        println!("Successfully created module directory: {module_path:?}");
        create_module_file(&module_path);
        true
    }
    Err(e) => {
        eprintln!("Failed to create module directory: {e}");
        false
    }
}