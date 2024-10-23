fn main() {

    let t1 = (10,12,1212.12,true,'A',"Hello World");
    println!("0:{},1:{},2:{},3:{},4:{},5:{}",t1.0,t1.1,t1.2,t1.3,t1.4,t1.5);

    let(num1,num2,float1,ok1,char1,str1)=t1;
    println!("num1:{},num2:{},float1:{},ok1:{},char1:{},str1:{}",num1,num2,float1,ok1,char1,str1);
    // while creating the touple , distributed to different variables
    let (n1,n2) = (10,20);
    println!("n1:{} n2:{}",n1,n2);

    let t2: (f64, f64)=calc_area_perimeter(12.123,123.12);
    println!("area:{:.2} perimeter:{:.2}",t2.0,t2.1);

    let (a1,p1)=calc_area_perimeter(12.123,123.12);
    println!("area:{:.2} perimeter:{:.2}",a1,p1);

    println!("t2:{:?}",t2);

    let t3=(); // unit touple

}

fn calc_area_perimeter(l:f32,b:f32)->(f64,f64){
    let area= (l * b)as f64 ;
    let perimeter:f64 = (2.0 *(l+b))as f64;
    (area,perimeter)
}

// collection of values os same type or different type are grouple together , we call it touple 
// write a simple function, to take two values a,b and return (add,sub,mul,divide) as a touple