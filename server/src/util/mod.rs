use uuid::Uuid;

mod util;

pub fn uuid() -> String {
    let uuid = Uuid::new_v4();
    let uuid_string = uuid.to_string().replace("-", "");
    // println!("{}", uuid_string);
    return uuid_string;
}