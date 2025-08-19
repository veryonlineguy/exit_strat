pub fn get_menu() -> String {
    let ingredients = vec![
        "1 cup Botan Rice",
        "15g ghee",
        "2 tsp better than buillon",
        "25g corn",
    ];

    let mut menu = String::from("## Caffeine\n- [ ] 2 tsp instant coffee\n- [ ] 2tsp half calf\n- [ ] green tea\n- [ ] green tea\n- [ ] green tea\n## Instant Pot\n");
    for ing in &ingredients {
        menu.push_str(&("- [ ] ".to_owned() + ing + "\n"));
    }
    menu.push_str("\n\n### Add Ons\n- [ ] Creatine\n- [ ] Collagen\n- [ ] can of chicken\n- [ ] can of chicken\n\n## Instructions\n");
    let water = "Water: 2 cups\n";
    let ip = "IP: Rice settingn\n";

    menu.push_str(&water);
    menu.push_str(&ip);
    String::from(menu)
}
