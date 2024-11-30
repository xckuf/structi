use crate::enums::enum_email::EmailDomain;
pub async fn valid_email(email: String) -> Result<(), String> {
    let allowed_domains = [
        EmailDomain::MailRu,
        EmailDomain::YandexRu,
        EmailDomain::GmailCom,
    ];

    if allowed_domains.iter().any(|domain| email.ends_with(domain.as_str())) {
        Ok(())
    } else {
        Err(format!(
            "Email должен заканчиваться на {}",
            allowed_domains
                .iter()
                .map(|domain| domain.as_str())
                .collect::<Vec<&str>>()
                .join(", ")
        ))
    }
}