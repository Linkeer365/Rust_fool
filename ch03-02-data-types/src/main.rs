fn main() {
    let tup=(1,3.5,"str");
    let (x,y,z)=tup;
    let arr_succ=[1,2,3,4,5];
    let arr_same=[3;5];
    // 表示3这个数有5个；
    println!("arr_succ[0]:{}",arr_succ[0]);
    println!("arr_same[0]:{}",arr_same[4]);
    println!("x:{},tup.0:{}",x,tup.0);
    println!("y:{},tup.1:{}",y,tup.1);
    println!("z:{},tup.2:{}",z,tup.2);
}
