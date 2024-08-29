use std::{env, fmt::Debug, fs::{self, File}, io::Write, process::exit, time::Duration};

use serde_json::Value;


#[derive(Debug)]
#[allow(dead_code)]
struct UserData{
    bid: i64,
    name: String,
    intro: String,
    birthday: String,
    gender: String,
    region: String,
}

impl UserData {
    fn to_string(&self) -> String {
        format!("{}, {}, {}, {}, {}, {}\n", self.bid, self.name, self.intro.replace('\n', " ").replace(',', "，"), self.birthday, self.gender, self.region)
    }
}

fn get_json_number(json: &serde_json::Value, key: &str) -> Option<i64> {
    json.get(key)?.as_i64()
}

fn get_json_string<'a>(json: &'a serde_json::Value, key: &str) -> Option<&'a str> {
    json.get(key)?.as_str()
}

async fn get_user_profile(bid: &str) -> Result<Value, reqwest::Error>{
    Ok(reqwest::get(format!("https://code-api-pc.dao3.fun/user/profile/{bid}"))
        .await?
        .json::<serde_json::Value>()
        .await?
    )
}

async fn get_user_info(bid: &str) -> Result<Value, reqwest::Error>{
    Ok(reqwest::get(format!("https://code-api-pc.dao3.fun/user/profile-info?userId={bid}"))
        .await?
        .json::<serde_json::Value>()
        .await?
    )
}
fn check_res(res: &Value) -> bool{
    match get_json_number(&res, "code"){
        Some(200) => true,
        _ => false
    }
}

fn parse_user_profile(profile: &Value, info: &Value) -> Result<UserData, &'static str> {
    let data = match profile.get("data") {
        Some(data) => Ok(data),
        _ => Err("profile data not found")
    }?;

    let bid = match get_json_number(data, "userId") {
        Some(bid) => Ok(bid),
        _ => Err("bid not found")
    }?;

    let name = match get_json_string(data, "nickname") {
        Some(name) => Ok(name),
        _ => Err("nickname not found")
    }?;
    let name = String::from(name);

    let intro = match get_json_string(data, "introduction") {
        Some(intro) => intro,
        _ => "None"
    };
    let intro = String::from(intro);

    let data = match info.get("data") {
        Some(data) => Ok(data),
        _ => Err("info data not found")
    }?;

    let birthday = match get_json_string(data, "birthday") {
        Some(birthday) => birthday,
        _ => "Unknown"
    };
    let birthday = String::from(birthday);

    let gender = match get_json_number(data, "gender") {
        Some(sex) => sex,
        _ => 0
    };
    let gender = match gender {
        1 => "Male",
        2 => "Female",
        _ => "Unknown"
    };
    let gender = String::from(gender);

    let region = match get_json_string(data, "region") {
        Some(region) => region,
        _ => "Unknown"
    };
    let region = String::from(region);

    Ok(UserData { bid, name, intro, birthday, gender, region })
}

fn help() -> () {
    println!(r"
    Mapping Box3 user data.
    Usage:
        query <start_id> <end_id> query user data from start_id to end_id.
        merge                     merge all datas to one file.
        help                      show help.
    ");
}

async fn query(start_id: i32, end_id: i32) -> () {
    let mut csv_file = File::create(format!("box3-user-data-{}-to-{}.csv", start_id, end_id)).unwrap();
    csv_file.write(b"bid, name, intro, birthday, gender, region\n").unwrap();

    for i in start_id..end_id {
        let start = tokio::time::Instant::now();

        let bid = i.to_string();
        let profile = get_user_profile(&bid).await.unwrap();
        let profile_info = get_user_info(&bid).await.unwrap();

        let dur = tokio::time::Instant::now() - start;
        println!("id: {}, request time: {:?}", i, dur);

        // if dur >= Duration::from_millis(1500) {
        //     println!("request time too long. sleep 1min and continue.");
        //     tokio::time::sleep(std::time::Duration::from_secs(60)).await;
        // }

        if !check_res(&profile) || !check_res(&profile_info){
            continue;
        }
        let result = parse_user_profile(&profile, &profile_info).unwrap();
        println!("{:?}", result);
        csv_file.write(result.to_string().as_bytes()).unwrap();

        // 500ms is the best delay time according to test.
        tokio::time::sleep_until(start + Duration::from_millis(500)).await;
    }    
}

fn merge() -> () {
    let mut csv_file = File::create("box3-user-data.csv").unwrap();
    csv_file.write(b"bid, name, intro, birthday, gender, region\n").unwrap();


    // search all files by a regexp
    let files = fs::read_dir(env::current_dir().unwrap())
        .unwrap()
        .filter(|s| {
            match s {
                Ok(s) => s.file_name()
                    .into_string()
                    .unwrap()
                    .contains("box3-user-data-"),
                Err(_) => false
            }
        });
    
    for file in files {
        let lines = fs::read_to_string(file.unwrap().file_name());
        for line in lines.unwrap().lines() {
            if line == "bid, name, intro, birthday, gender, region" {
                continue;
            }
            csv_file.write(line.as_bytes()).unwrap();
            csv_file.write(b"\n").unwrap();
        }
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let cmd = args[1].parse::<String>();
    let cmd = match cmd {
        Ok(cmd) => cmd,
        Err(_) => {
            println!("Invalid command.");
            help();
            exit(1);
        }
    };
    match cmd.as_str() {
        "query" => {
            if args.len() != 4 {
                help();
                exit(1);
            }
            let start_id = args[2].parse::<i32>().unwrap();
            let end_id = args[3].parse::<i32>().unwrap();
            query(start_id, end_id).await;
        },
        "merge" => {
            merge();
        },
        _ => {
            help();
            exit(1);
        }
    }
}
