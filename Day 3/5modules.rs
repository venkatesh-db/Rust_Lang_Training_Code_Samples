
static NAME :&str = "shanthi nilya";

struct Invitation {
    date :String,
    location:String,
    time:String,
}

struct Families {
    mother :String,
    father:String,
    daughter:String,
    brother:String,
}

struct Pubtr<'a>{
    bustop:&str,
    f:Families,
}


fn main(){

    let rohan = Invitation{date:"2-june-2022".to_string(),location:"vasanth nagar".to_string(),time:"6 am".to_string()};
    
    let gowda = Families{mother:"seetha".to_string(), father :"rama".to_string() ,daughter:"rashmika".to_string(),brother:"govinda".to_string()};

    let bmtc = Pubtr{bustsop:"vasanthnagar",gowda };
    
}
