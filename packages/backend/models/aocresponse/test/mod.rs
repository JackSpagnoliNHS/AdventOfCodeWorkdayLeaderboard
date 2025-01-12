use std::io::BufReader;
use std::{collections::HashMap, fs::File};

use crate::models::aocresponse::{AOCMember, Day, DaysCompleted, ResponseMembers, TaskCompletion};

use super::AOCResponse;

#[test]
fn test_response_deserialisation() {
    let file =
        File::open("packages/backend/models/test/test_response.json").expect("Unable to open file");
    let reader = BufReader::new(file);

    let u: Result<AOCResponse, serde_json::Error> = serde_json::from_reader(reader);

    if u.is_err() {
        println!("{u:?}");
        panic!()
    }

    let response = u.unwrap();

    let member_2388937_day_1_1 = TaskCompletion {
        star_index: 89302,
        get_star_ts: 1669911189,
    };
    let member_2388937_day_1_2 = TaskCompletion {
        star_index: 90973,
        get_star_ts: 1669911672,
    };
    let mut member_2388937_day_1: Day = HashMap::new();
    member_2388937_day_1.insert("1".to_string(), member_2388937_day_1_1);
    member_2388937_day_1.insert("2".to_string(), member_2388937_day_1_2);

    let mut member_2388937_cdl: DaysCompleted = HashMap::new();
    member_2388937_cdl.insert("1".to_string(), member_2388937_day_1);

    let member_2388937 = AOCMember {
        global_score: 0,
        stars: 4,
        id: 2388937,
        name: "john-kieran-robson".to_string(),
        local_score: 18,
        last_star_ts: 1670427732,
        completion_day_level: member_2388937_cdl,
    };

    let member_2380866_day_3_1 = TaskCompletion {
        star_index: 602743,
        get_star_ts: 1670064301,
    };
    let member_2380866_day_3_2 = TaskCompletion {
        star_index: 608140,
        get_star_ts: 1670065607,
    };
    let member_2380866_day_16_1 = TaskCompletion {
        star_index: 3135956,
        get_star_ts: 1671190701,
    };

    let mut member_2380866_day_3: Day = HashMap::new();
    member_2380866_day_3.insert("1".to_string(), member_2380866_day_3_1);
    member_2380866_day_3.insert("2".to_string(), member_2380866_day_3_2);
    let mut member_2380866_day_16: Day = HashMap::new();
    member_2380866_day_16.insert("1".to_string(), member_2380866_day_16_1);

    let mut member_2380866_cdl: DaysCompleted = HashMap::new();
    member_2380866_cdl.insert("3".to_string(), member_2380866_day_3);
    member_2380866_cdl.insert("16".to_string(), member_2380866_day_16);

    let member_2380866 = AOCMember {
        global_score: 0,
        stars: 33,
        id: 2380866,
        name: "JackSpagnoli".to_string(),
        local_score: 422,
        last_star_ts: 1671835603,
        completion_day_level: member_2380866_cdl,
    };

    let mut expected_members: ResponseMembers = HashMap::new();
    expected_members.insert("2388937".to_string(), member_2388937);
    expected_members.insert("2380866".to_string(), member_2380866);
    let expected_response = AOCResponse {
        event: "2022".to_string(),
        owner_id: 1599442,
        members: expected_members,
    };

    assert_eq!(response, expected_response);
}
