

fn selection_sort(vec: &mut Vec<i32>){
    let n =vec.len();
    for i in 0..(n-1){
        let mut min = i;
        for j in (i+1)..n{
            if vec[j] < vec[min] {
                min = j;
            }
        if min != i{
            vec.swap(i,min)
        }
        }
    }
}


fn main() {
    let mut v:Vec<i32>= vec![14,15,2,3,9,4,1,2,5,-1,5,2,1,4,65,4,1,0];
    selection_sort(&mut v);
    println!("tableau trié : {:?}", v);

}
