use std::collections::HashMap;
fn main() {
    let mut map: HashMap<i32, &str> = HashMap::new();
    map.insert(560086, "Yeshvantpur , Bangalore");
    map.insert(560096, "Rajajinagar, Bangalore");
    map.insert(522001, "Guntur-1");
    map.insert(522002, "Guntur-2");
    map.insert(6459111, "Trivandrum");
    map.insert(2123243, "Some-A");
    map.insert(1234333, "Some-B");
    map.insert(4533232, "Some-C");

    for (key, value) in &mut map {
        if *key == 522001 {
            *value = "Guntur-1, Andhra Pradesh";
        }
        println!("Key:{} Value:{}", key, *value);
    }
    println!("Len of the map:{}", map.len());
    println!("Cap of the map:{}", map.capacity());
    let k = 522001;
    match map.get(&k) {
        Some(&s1) => {
            println!("Some Value:{}", s1);
        }
        None => {
            println!("No value for the key provided");
        }
    }
    map.insert(522003, "Guntur-3");
    map.entry(522003).or_insert("Guntur-3");
    //map.entry(522001).and_modify("Guntur-1,Andhra Pradesh");
    println!("map:{:#?}", map);

    map.remove(&522003);

    println!("map:{:#?}", map);
}

// maps are key value pairs
// values are mapped to keys
// O(1) --> constant times
// 0a4d55a8d778e5022fa b701977c5d840bbc486d0 -> Hello World
// 0a4d55a8d778e5022fab701977c5d840bbc486d0
// 281ce516d31593280325 473d5cb5340c1983a83f
// hashkey -> 4            2
/*
|  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |
| 0|1 |2 |3 |4 |5 |6 |7 |8 |
*/
