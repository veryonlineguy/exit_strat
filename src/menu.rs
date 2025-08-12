pub fn get_menu() -> String {
    let ingredients = vec![
        "1 cup Botan Rice",
        "15g ghee",
        "1 tsp better than buillon",
        "1 can of tomato sauce",
    ];

    let mut menu = String::from("## Instant Pot\n");
    for ing in &ingredients {
        menu.push_str(&("- [ ] ".to_owned() + ing + "\n"));
    }
    menu.push_str("\n\n### Add Ons\n- [ ] Creatine\n- [ ] Collagen\n- [ ] can of chicken\n- [ ] can of chicken\n- [ ] bag berries\n\n## Instructions\n");
    let water = "Water: 8 cups\n";
    let ip = "IP: 20 min\n\n";

    menu.push_str(&water);
    menu.push_str(&ip);
    String::from(menu)
}
