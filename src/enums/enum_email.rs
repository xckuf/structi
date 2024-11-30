pub enum EmailDomain {
    MailRu,
    YandexRu,
    GmailCom,
}

impl EmailDomain {
    pub fn as_str(&self) -> &'static str {
        match self {
            EmailDomain::MailRu => "@mail.ru",
            EmailDomain::YandexRu => "@yandex.ru",
            EmailDomain::GmailCom => "@gmail.com",
        }
    }
}