use std::collections::HashMap;

#[derive(Debug)]
struct Status {
    mp: u32,
    w: u32,
    d: u32,
    l: u32,
    p: u32,
}

fn update_win(hm: &mut HashMap<String, Status>, team1: String, team2: String) {
    hm.entry(team1)
        .and_modify(|e| {
            e.mp += 1;
            e.w += 1;
            e.p += 3;
        })
        .or_insert(Status {
            mp: 1,
            w: 1,
            d: 0,
            l: 0,
            p: 3,
        });

    hm.entry(team2)
        .and_modify(|e| {
            e.mp += 1;
            e.l += 1;
        })
        .or_insert(Status {
            mp: 1,
            w: 0,
            d: 0,
            l: 1,
            p: 0,
        });
}

fn update_draw(hm: &mut HashMap<String, Status>, team1: String, team2: String) {
    hm.entry(team1)
        .and_modify(|e| {
            e.mp += 1;
            e.d += 1;
            e.p += 1;
        })
        .or_insert(Status {
            mp: 1,
            w: 0,
            d: 1,
            l: 0,
            p: 1,
        });

    hm.entry(team2)
        .and_modify(|e| {
            e.mp += 1;
            e.d += 1;
            e.p += 1;
        })
        .or_insert(Status {
            mp: 1,
            w: 0,
            d: 1,
            l: 0,
            p: 1,
        });
}

fn update_status(hm: &mut HashMap<String, Status>, team1: String, team2: String, status: &str) {
    match status {
        "win" => update_win(hm, team1, team2),
        "draw" => update_draw(hm, team1, team2),
        "loss" => update_win(hm, team2, team1),
        _ => panic!("Undefined status: {:?}", status),
    }
}

fn generate_output(hm: HashMap<String, Status>) -> String {
    let mut res: Vec<String> = vec![String::from(
        "Team                           | MP |  W |  D |  L |  P",
    )];

    let mut v = hm.iter().collect::<Vec<(&String, &Status)>>();
    v.sort_unstable_by(|a, b| {
        if a.1.p == b.1.p {
            a.0.partial_cmp(&b.0).unwrap()
        } else {
            b.1.p.partial_cmp(&a.1.p).unwrap()
        }
    });
    for (name, st) in v.iter() {
        res.push(format!(
            "{:31}| {:2} | {:2} | {:2} | {:2} | {:2}",
            name, st.mp, st.w, st.d, st.l, st.p
        ));
    }
    res.join("\n")
}

pub fn tally(match_results: &str) -> String {
    let mut hm: HashMap<String, Status> = HashMap::new();
    for line in match_results.lines() {
        let split = line.split(';').collect::<Vec<&str>>();
        let team1 = split[0].into();
        let team2 = split[1].into();
        update_status(&mut hm, team1, team2, split[2]);
    }

    generate_output(hm)
}
