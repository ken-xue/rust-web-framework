use uuid::Uuid;
pub mod util;
pub mod encrypt;

pub fn uuid() -> String {
    let uuid = Uuid::new_v4();
    let uuid_string = uuid.to_string().replace("-", "");
    return uuid_string;
}