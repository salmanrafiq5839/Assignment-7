mod model {
    pub mod car {
        #[derive(Debug)]
        pub struct Data {
            name:String
        }
        #[derive(Debug)]
        pub enum car_Type{
            Automatic(String),
            Manual(String)
        }
    }
    #[derive(Debug)]
    pub mod company {
        pub fn car_type(){
            println!("this is beautiful car ");
        }

    }
}

use crate::model::car;

fn main() {
   model::company::car_type();

    let data1 = model::car::Data {
        name:"FAW".to_string()
    };
    println!("{:?}",data1);
}
