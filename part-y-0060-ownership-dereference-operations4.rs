fn main() {

    let emp_name = "Ayhan Bilir".to_string();

    let birth: u16 = 1976;

    let mut emp_ayhan_salary_usd = 3200;

    let emp_ayhan_data = set_person(&emp_name, birth, true, &mut emp_ayhan_salary_usd, 0.13);

    println!("Emp Ayhan data: {:?}", emp_ayhan_data);

    println!("{emp_ayhan_salary_usd}");
}


fn set_person(full_name: &String, birth_year: u16, is_married: bool, salary_usd: &mut u16, salary_inc_rate: f32) -> (String, u16, bool, f32) {

    let new_salary = *salary_usd as f32 * (1.0 + salary_inc_rate);

    *salary_usd = new_salary as u16;

    (full_name.to_string(), birth_year, is_married, new_salary)
}

// Emp Ayhan data: ("Ayhan Bilir", 1976, true, 3616.0)
// 3616
