use ada_playbooks::recipes::RecipeRunner;
use std::path::Path;

#[test]
fn test_recipe_parsing() {
    let recipe_path = Path::new("../../playbooks/build_recipes/electron_build.yaml");
    
    // Test if file exists relative to workspace root vs crate root depending on cargo test cwd
    let path = if recipe_path.exists() {
        recipe_path
    } else {
        Path::new("playbooks/build_recipes/electron_build.yaml")
    };

    if path.exists() {
        let recipe = RecipeRunner::parse(path).expect("Failed to parse electron recipe");
        assert_eq!(recipe.id, "R_ELECTRON_DESKTOP");
        assert_eq!(recipe.platforms, vec!["windows"]);
        assert_eq!(recipe.steps[0].name, "Install Dependencies");
        assert_eq!(recipe.smoke_checks[0].expected_exit_code, 0);
    }
}
