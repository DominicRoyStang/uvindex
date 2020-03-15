use prettytable::{Table, Row, Cell, format};
use textwrap;

pub fn print_info_table(max_width: usize) {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_BOX_CHARS);

    let fixed_size = 27;
    let variable_size = max_width - fixed_size;

    table.set_titles(Row::new(vec![Cell::new("UV Index"), Cell::new("Risk"), Cell::new("Recommended Protection")]));

    for row in info_data() {
        let (index_range, risk, recommended_protection) = row;
        let index_range_cell = Cell::new(&index_range);
        let risk_cell = Cell::new(&risk);
        let recommended_protection_cell = Cell::new(&textwrap::fill(&recommended_protection, variable_size));
        table.add_row(Row::new(vec![index_range_cell, risk_cell, recommended_protection_cell]));
    }

    table.printstd();
}

pub fn print_info_text() {
    let mut text = String::from("");

    for row in info_data() {
        let (index_range, risk, recommended_protection) = row;
        text.push_str(&format!("UV Index: {}. Risk: {}.\n{}\n\n", index_range, risk, recommended_protection));
    }

    println!("{}", text);
}

fn info_data() -> Vec<(String, String, String)> {
    return vec![
        (String::from("0 to 2"), String::from("Low"), low_risk_message()),
        (String::from("3 to 5"), String::from("Moderate"), moderate_risk_message()),
        (String::from("6 to 7"), String::from("High"), high_risk_message()),
        (String::from("8 to 10"), String::from("Very High"), very_high_risk_message()),
        (String::from("11+"), String::from("Extreme"), extreme_risk_message())
    ];
}

fn low_risk_message() -> String {
    return String::from(
        "A UV index reading of 0 to 2 means low danger from the Sun's UV rays for the average person.\n\
        Wear sunglasses on bright days. If you burn easily, cover up and use broad spectrum SPF 30+ sunscreen. \
        Bright surfaces, such as sand, water, and snow, will increase UV exposure."
    );
}

fn moderate_risk_message() -> String {
    return String::from(
        "A UV index reading of 3 to 5 means moderate risk of harm from unprotected Sun exposure.\n\
        Stay in shade near midday when the Sun is strongest. \
        If outdoors, wear Sun protective clothing, a wide-brimmed hat, and UV-blocking sunglasses. \
        Generously apply broad spectrum SPF 30+ sunscreen every 2 hours, even on cloudy days, and after swimming or sweating. \
        Bright surfaces, such as sand, water, and snow, will increase UV exposure."
    );
}

fn high_risk_message() -> String {
    return String::from(
        "A UV index reading of 6 to 7 means high risk of harm from unprotected Sun exposure. \
        Protection against skin and eye damage is needed.\n\
        Reduce time in the Sun between 10 a.m. and 4 p.m. \
        If outdoors, seek shade and wear Sun protective clothing, a wide-brimmed hat, and UV-blocking sunglasses. \
        Generously apply broad spectrum SPF 30+ sunscreen every 2 hours, even on cloudy days, and after swimming or sweating. \
        Bright surfaces, such as sand, water, and snow, will increase UV exposure."
    );
}

fn very_high_risk_message() -> String {
    return String::from(
        "A UV index reading of 8 to 10 means very high risk of harm from unprotected Sun exposure. \
        Take extra precautions because unprotected skin and eyes will be damaged and can burn quickly.\n\
        Minimize Sun exposure between 10 a.m. and 4 p.m. \
        If outdoors, seek shade and wear Sun protective clothing, a wide-brimmed hat, and UV-blocking sunglasses. \
        Generously apply broad spectrum SPF 30+ sunscreen every 2 hours, even on cloudy days, and after swimming or sweating. \
        Bright surfaces, such as sand, water, and snow, will increase UV exposure."
    );
}

fn extreme_risk_message() -> String {
    return String::from(
        "A UV index reading of 11 or more means extreme risk of harm from unprotected Sun exposure. \
        Take all precautions because unprotected skin and eyes can burn in minutes.\n\
        Try to avoid Sun exposure between 10 a.m. and 4 p.m.\
        If outdoors, seek shade and wear Sun protective clothing, a wide-brimmed hat, and UV-blocking sunglasses. \
        Generously apply broad spectrum SPF 30+ sunscreen every 2 hours, even on cloudy days, and after swimming or sweating. \
        Bright surfaces, such as sand, water, and snow, will increase UV exposure."
    );
}
