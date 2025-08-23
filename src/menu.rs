pub fn get_menu() -> String {
    let ingredients = vec![
        "1 cup Botan Rice",
        "15g ghee",
        "2 tsp better than buillon",
    ];

    let mut menu = String::from("## Caffeine\n- [ ] Hario Pour over\n- [ ] Hario Pour over\n## Instant Pot\n");
    for ing in &ingredients {
        menu.push_str(&("- [ ] ".to_owned() + ing + "\n"));
    }
    menu.push_str("### Add Ons\n- [ ] 30g blueberry\n- [ ] 30g blueberry\n- [ ] 30g bluberry\n## Instructions\n");
    let water = "Water: 8 cups\n";
    let ip = "IP: Rice settingn\n";

    menu.push_str(&water);
    menu.push_str(&ip);
    String::from(menu)
}
