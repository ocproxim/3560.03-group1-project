use crate::models::sport::Sport;

pub fn generate_sports() -> Vec<Sport> {
    let basketball_id = 0;
    let basketball_name = "Basketball".to_string();
    let basketball = Sport {
        sportID: basketball_id,
        sportName: basketball_name,
    };

    let soccer_id = 1;
    let soccer_name = "Soccer".to_string();
    let soccer = Sport {
        sportID: soccer_id,
        sportName: soccer_name,
    };

    let baseball_id = 2;
    let baseball_name = "Baseball".to_string();
    let baseball = Sport {
        sportID: baseball_id,
        sportName: baseball_name,
    };

    vec![basketball, soccer, baseball]
}
