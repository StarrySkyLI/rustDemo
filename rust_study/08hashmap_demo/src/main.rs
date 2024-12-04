use std::collections::HashMap;

fn main() {
    //创建hashmap的第一种方法
    let s1=String::from("s1");
    let s2=String::from("s2");
    let s3=String::from("s1");
    let s4=String::from("s2");
    let mut scores = HashMap::new();
    scores.insert(String::from("bule"), 10);
    scores.insert(String::from("red"), 20);
    let mut sc2=HashMap::new();
    sc2.insert(s1,s2);//s1，s2会被拿到所有权失效
    let mut sc3=HashMap::new();
    sc3.insert(&s3,&s4);//引用不会
    // 第二种
    let teams = vec![String::from("blue"), String::from("yellow")];
    let intial_scores = vec![10, 50];
    let scores: HashMap<_, _> =
        teams.iter().zip(intial_scores.iter()).collect();
    println!("{:?}", scores);
    // 获取hashmap的值
    let team_name =String::from("blue");
    let score=scores.get(&team_name);

    match score {
        None => {println!("no exit")}
        Some(std) => {println!("num is {}",std)}
    }
    //遍历hashmap
    for (k,v)in &scores{
        println!("{}:{}",k,v);
    }
    //更新kv
    let text ="hello world ! what a wonderful world !";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count =map.entry(word).or_insert(0);//entry()检查指定的key是否存在对应一个V
        // or_insert()k存在返回v对应的一个可变引用，k不存在将k插入再返回这个值的可变引用
        *count +=1;
    }
    println!("{:#?}",map)
}
