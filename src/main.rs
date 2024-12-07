fn main() {
    {
        let  sample = "this is sample";
        println!("****the value of sample:{sample}***");
    }
    //下面的語句無法執行。因為sample已經離開它自己的作用域，已經被內存回收了
    //println!("****the value of sample:{sample}***");
}
