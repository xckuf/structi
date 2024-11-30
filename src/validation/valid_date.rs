pub async fn valid_date(date: String) -> Result<(), String> {
    let parts: Vec<&str> = date.split('.').collect();

    if parts.len() != 3 {
        return Err("Дата должна быть в формате ДД.ММ.ГГГГ".to_string());
    }

    let day: u32 = match parts[0].parse() {
        Ok(d) => d,
        Err(_) => return Err("День должен быть числом".to_string()),
    };

    let month: u32 = match parts[1].parse() {
        Ok(m) => m,
        Err(_) => return Err("Месяц должен быть числом".to_string()),
    };

    let year: u32 = match parts[2].parse() {
        Ok(y) => y,
        Err(_) => return Err("Год должен быть числом".to_string()),
    };

    if day < 1 || day > 31 {
        return Err("День должен быть в диапазоне от 1 до 31".to_string());
    }

    if month < 1 || month > 12 {
        return Err("Месяц должен быть в диапазоне от 1 до 12".to_string());
    }

    if year > 2025 {
        return Err("Год не может быть больше 2025".to_string());
    }

    if month == 2 && day > 29 {
        return Err("Февраль не может содержать более 29 дней".to_string());
    }

    let days_in_month = match month {
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 31,
    };

    if day > days_in_month {
        return Err(format!(
            "Месяц {} не может содержать более {} дней",
            month, days_in_month
        ));
    }
    Ok(())
}

fn is_leap_year(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}