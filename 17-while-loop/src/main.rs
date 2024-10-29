fn main() {
    let mut counter = 1;

    while (counter <= 10) {
        println!("{}", counter);
        counter += 1;
    }

    let mut num = 2;
    let mut range = 100;
    println!("prime numbers");
    print!("1 ");
    while (num <= range) {

        let mut index =2;
        let mut counter=0;

        while(index<num){
            if num%index==0{
                counter+=1;
                break;
            }
            index+=1;
        }
        if counter==0{
            print!("{} ",num);
        }

        num += 1;
    }
}
