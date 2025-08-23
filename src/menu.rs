pub fn get_menu() -> String {
    let ingredients = vec![
        "3/2 cup Botan Rice",
        "20g ghee",
        "2 tsp better than buillon",
        "100g frozen broccoli",
        "can of tommatoes",
        "1/4 cup of mung beans"
    ];

    let mut menu = String::from("## Caffeine\n- [ ] Hario Pour over\n- [ ] Hario Pour over\n## Instant Pot\n");
    for ing in &ingredients {
        menu.push_str(&("- [ ] ".to_owned() + ing + "\n"));
    }
    menu.push_str("### Add Ons\n- [ ] 30g blueberry\n- [ ] 30g blueberry\n- [ ] 30g bluberry\n## Instructions\n");
    let water = "Water: 12 cups\n";
    let ip = "IP: 60 min\n";

    menu.push_str(&water);
    menu.push_str(&ip);
    String::from(menu)
}
